use crate::Cmd;
use crate::KvsEngine;
use crate::Result;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
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

#[derive(Clone)]
pub struct KvStore {
    inner: Arc<Mutex<KvStoreInner>>,
}

struct KvStoreInner {
    store: HashMap<String, LogPointer>,
    reader: HashMap<u64, BufReader<File>>,
    current_file_id: u64,
    writer: BufWriter<File>,
    uncompacted_bytes: u64,
    dir_path: PathBuf,
}

impl KvStore {
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

        let inner = KvStoreInner {
            store: index,
            reader: readers,
            writer,
            current_file_id,
            uncompacted_bytes: uncompacted,
            dir_path: dir,
        };
        Ok(KvStore {
            inner: Arc::new(Mutex::new(inner)),
        })
    }
}

impl KvStoreInner {
    pub fn compact(&mut self) -> Result<()> {
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
        //
        let mut new_offset: u64 = 0;

        let KvStoreInner {
            ref mut store,
            ref mut reader,
            ..
        } = *self;

        for (_key, log_ptr) in store.iter_mut() {
            // Read the old command from the old file
            let r = reader
                .get_mut(&log_ptr.file_id)
                .ok_or_else(|| failure::err_msg("reader not found"))?;
            r.seek(SeekFrom::Start(log_ptr.offset))?;
            let mut buf = vec![0u8; log_ptr.length as usize];
            r.read_exact(&mut buf)?;

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
}

impl KvsEngine for KvStore {
    fn set(&self, key: String, value: String) -> Result<()> {
        let mut inner = self.inner.lock().unwrap();
        let cmd: Cmd = Cmd::Set {
            key: key.clone(),
            value: value.clone(),
        };
        let serialized = serde_json::to_string(&cmd)?;
        let offset = inner.writer.seek(SeekFrom::Current(0))?;
        writeln!(inner.writer, "{}", serialized)?;
        inner.writer.flush()?;
        let length = serialized.len() as u64 + 1;
        let file_id = inner.current_file_id;
        if let Some(old_ptr) = inner.store.insert(
            key,
            LogPointer {
                offset,
                length,
                file_id,
            },
        ) {
            inner.uncompacted_bytes += old_ptr.length;
        }
        if inner.uncompacted_bytes > COMPACTION_THRESHOLD {
            inner.compact()?;
        }
        Ok(())
    }
    fn get(&self, key: String) -> Result<Option<String>> {
        let mut inner = self.inner.lock().unwrap();
        let log_ptr = match inner.store.get(&key) {
            None => return Ok(None),
            Some(ptr) => LogPointer {
                offset: ptr.offset,
                length: ptr.length,
                file_id: ptr.file_id,
            },
        };
        let reader = inner
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

    fn remove(&self, key: String) -> Result<()> {
        let mut inner = self.inner.lock().unwrap();
        if !inner.store.contains_key(&key) {
            return Err(failure::err_msg("Key not found"));
        }
        let cmd = Cmd::Rm { key: key.clone() };
        let serialized = serde_json::to_string(&cmd)?;
        writeln!(inner.writer, "{}", serialized)?;
        inner.writer.flush()?;
        if let Some(old_ptr) = inner.store.remove(&key) {
            inner.uncompacted_bytes += old_ptr.length;
        }
        inner.uncompacted_bytes += serialized.len() as u64 + 1;
        if inner.uncompacted_bytes > COMPACTION_THRESHOLD {
            inner.compact()?;
        }
        Ok(())
    }
}

fn log_pathe(dir: &PathBuf, file_id: u64) -> PathBuf {
    dir.join(format!("{}.log", file_id))
}
