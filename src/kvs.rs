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
}
/// # A implementatoin of Key Value Store

pub struct KvStore {
    store: HashMap<String, LogPointer>,
    // log_path: PathBuf,
    reader: BufReader<File>,
    writer: BufWriter<File>,
}

impl KvStore {
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
        self.store.insert(key, LogPointer { offset, length });
        Ok(())
    }
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        match self.store.get(&key) {
            None => Ok(None),
            Some(log_ptr) => {
                self.reader.seek(SeekFrom::Start(log_ptr.offset))?;
                let mut buf = vec![0u8; log_ptr.length as usize];
                self.reader.read_exact(&mut buf)?;
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
        self.store.remove(&key);
        Ok(())
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let dir = path.into();
        let log_path = dir.join("log");
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)?;
        let file = File::open(&log_path)?;
        let mut replay_reader = BufReader::new(file);
        let mut index = HashMap::new();
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
                    index.insert(
                        key,
                        LogPointer {
                            offset: pos,
                            length: bytes_read as u64,
                        },
                    );
                }
                Cmd::Rm { key } => {
                    index.remove(&key);
                }
            }
            pos += bytes_read as u64;
        }
        let mut writer = BufWriter::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_path)?,
        );
        writer.seek(SeekFrom::End(0))?;
        let reader = BufReader::new(File::open(&log_path)?);
        Ok(KvStore {
            store: index,
            // log_path,
            reader,
            writer,
        })
    }
}
