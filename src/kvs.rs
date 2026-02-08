use crate::Cmd;
use crate::Result;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};
// 16 bytes
struct LogPointer {
    // byte position where the command starts
    offset: u64,
    // how mnay bytes the serialized command is
    length: u64,
    file_id: u64,
}

const COMPACTION_THRESHOLD: u64 = 1024 * 1024;
/// # A implementatoin of Key Value Store

pub struct KvStore {
    store: HashMap<String, LogPointer>,
    // log_path: PathBuf,
    reader: HashMap<u64, BufReader<File>>,
    // what file we are appending to
    current_file_id: u64,
    writer: BufWriter<File>,
    //how many bytes of dead entries exist
    uncompacted_bytes: u64,
    dir_path: PathBuf,
}

impl KvStore {
    fn compact(&mut self) -> Result<()> {
        // Step A: Pick new file IDs
        let compaction_file_id = self.current_file_id + 1;
        let new_writer_file_id = self.current_file_id + 2;

        // Step B: Create the compaction file and its writer
        let compact_path = log_pathe(&self.dir_path, compaction_file_id);
        let mut compact_writer = BufWriter::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(&compact_path)?,
        );

        // Step C: Write all live entries to the compaction file
        let mut new_offset: u64 = 0;
        for log_ptr in self.store.values_mut() {
            // Read the old command from the old file
            let reader = self
                .reader
                .get_mut(&log_ptr.file_id)
                .ok_or_else(|| failure::err_msg("reader not found"))?;
            reader.seek(SeekFrom::Start(log_ptr.offset))?;
            let mut buf = vec![0u8; log_ptr.length as usize];
            reader.read_exact(&mut buf)?;

            // Write it to the compaction file
            compact_writer.write_all(&buf)?;

            // Update this LogPointer IN PLACE to point to the new location
            log_ptr.offset = new_offset;
            log_ptr.length = buf.len() as u64;
            log_ptr.file_id = compaction_file_id;

            new_offset += buf.len() as u64;
        }
        compact_writer.flush()?;

        // Step D: Collect the old file IDs we need to delete
        let old_file_ids: Vec<u64> = self.reader.keys().copied().collect();

        // Step E: Remove old readers and delete old files
        for old_id in old_file_ids {
            self.reader.remove(&old_id);
            std::fs::remove_file(log_pathe(&self.dir_path, old_id))?;
        }

        // Step F: Open a reader for the compaction file
        self.reader.insert(
            compaction_file_id,
            BufReader::new(File::open(&compact_path)?),
        );

        // Step G: Create the new writer file for future writes
        self.current_file_id = new_writer_file_id;
        let new_writer_path = log_pathe(&self.dir_path, new_writer_file_id);
        self.writer = BufWriter::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(&new_writer_path)?,
        );
        self.reader.insert(
            new_writer_file_id,
            BufReader::new(File::open(&new_writer_path)?),
        );

        // Step H: Reset the dead byte counter
        self.uncompacted_bytes = 0;

        Ok(())
    }
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd: Cmd = Cmd::Set {
            key: key.clone(),
            value: value.clone(),
        };
        let serialized = serde_json::to_string(&cmd)?;
        let offset = self.writer.seek(SeekFrom::Current(0))?;
        writeln!(self.writer, "{}", serialized)?;
        self.writer.flush()?;
        let length = serialized.len() as u64 + 1;
        if let Some(old_ptr) = self.store.insert(
            key,
            LogPointer {
                offset,
                length,
                file_id: self.current_file_id,
            },
        ) {
            self.uncompacted_bytes += old_ptr.length;
        }
        if self.uncompacted_bytes > COMPACTION_THRESHOLD {
            self.compact()?;
        }
        Ok(())
    }
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        match self.store.get(&key) {
            None => Ok(None),
            Some(log_ptr) => {
                let reader = self
                    .reader
                    .get_mut(&log_ptr.file_id)
                    .ok_or_else(|| failure::err_msg("Log file not found"))?;
                reader.seek(SeekFrom::Start(log_ptr.offset))?;
                let mut buf = vec![0u8; log_ptr.length as usize];
                reader.read_exact(&mut buf)?;
                let line = String::from_utf8(buf)?;
                let cmd: Cmd = serde_json::from_str(line.trim())?;
                match cmd {
                    Cmd::Set { value, .. } => Ok(Some(value)),
                    Cmd::Rm { .. } => Ok(None),
                }
            }
        }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        if !self.store.contains_key(&key) {
            return Err(failure::err_msg("Key not found"));
        }
        let cmd = Cmd::Rm { key: key.clone() };
        let serialized = serde_json::to_string(&cmd)?;
        writeln!(self.writer, "{}", serialized)?;
        self.writer.flush()?;
        if let Some(old_ptr) = self.store.remove(&key) {
            self.uncompacted_bytes += old_ptr.length;
        }
        self.uncompacted_bytes += serialized.len() as u64 + 1;
        if self.uncompacted_bytes > COMPACTION_THRESHOLD {
            self.compact()?;
        }
        Ok(())
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let dir = path.into();
        let mut file_ids: Vec<u64> = std::fs::read_dir(&dir)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let name = entry.file_name().into_string().ok()?;
                if name.ends_with(".log") {
                    name.trim_end_matches(".log").parse::<u64>().ok()
                } else {
                    None
                }
            })
            .collect();
        file_ids.sort();

        let mut index = HashMap::new();
        let mut readers = HashMap::new();
        let mut uncompacted: u64 = 0;
        for &fid in &file_ids {
            let fpath = log_pathe(&dir, fid);
            let file = File::open(&fpath)?;

            let mut replay_reader = BufReader::new(file);
            let mut pos: u64 = 0;
            loop {
                let mut line = String::new();
                let bytes_read = replay_reader.read_line(&mut line)?;
                if bytes_read == 0 {
                    break;
                }
                let cmd: Cmd = serde_json::from_str(line.trim())?;
                match cmd {
                    Cmd::Set { key, .. } => {
                        if let Some(old_ptr) = index.insert(
                            key,
                            LogPointer {
                                offset: pos,
                                length: bytes_read as u64,
                                file_id: fid,
                            },
                        ) {
                            uncompacted += old_ptr.length;
                        }
                    }
                    Cmd::Rm { key } => {
                        if let Some(old_ptr) = index.remove(&key) {
                            uncompacted += old_ptr.length;
                        }
                        uncompacted += bytes_read as u64;
                    }
                }
                pos += bytes_read as u64;
            }
            readers.insert(fid, BufReader::new(File::open(&fpath)?));
        }
        let current_file_id = file_ids.last().copied().unwrap_or(0) + 1;
        let writer_path = log_pathe(&dir, current_file_id);
        let writer = BufWriter::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(&writer_path)?,
        );
        readers.insert(current_file_id, BufReader::new(File::open(&writer_path)?));

        Ok(KvStore {
            store: index,
            reader: readers,
            writer,
            current_file_id,
            uncompacted_bytes: uncompacted,
            dir_path: dir,
        })
    }
}

fn log_pathe(dir: &PathBuf, file_id: u64) -> PathBuf {
    dir.join(format!("{}.log", file_id))
}
