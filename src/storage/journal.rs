// Created by AG on 15-08-2026

use crate::types::{SlimeDBError, Result};
use parking_lot::Mutex;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::sync::atomic::AtomicU64;

const JOURNAL_SIGNATURE: &[u8; 5] = b"SLIME";
const JOURNAL_RECORD_HEADER_SIZE: usize = 12;

pub struct Journal {
    file_path: PathBuf,
    writer: Mutex<BufWriter<File>>,
    sequence: AtomicU64,
    sync_on_write: bool,
}

impl Journal {
    pub fn open(file_path: PathBuf, sync_on_write: bool) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(&file_path)?;

        let mut file_reader = BufReader::new(file.try_clone()?);
        let read_sequence = Self::recover_write_sequence(&mut file_reader)?;
        let journal_writer = BufWriter::with_capacity(64 * 1024, file);

        Ok(
            Self {
                file_path,
                writer: Mutex::new(journal_writer),
                sequence: AtomicU64::new(read_sequence),
                sync_on_write,
            }
        )
    }

    fn recover_write_sequence(reader: &mut BufReader<File>) -> Result <u64> {
        let file_length = reader.seek(SeekFrom::End(0))?;

        if file_length == 0 {
            return Ok(0);
        }
        reader.seek(SeekFrom::Start(0))?;

        let mut journal_sig = [0u8; 5];
        if reader.read_exact(&mut journal_sig).is_ok() && &journal_sig != JOURNAL_SIGNATURE {
            return Err(SlimeDBError::Corruption("Invalid Journal Signature".to_string()));
        }

        let mut max_sequence = 0u64;
        reader.seek(SeekFrom::Start(4))?;

        loop {
            let mut file_header = [0u8; JOURNAL_RECORD_HEADER_SIZE];
            match reader.read_exact(&mut file_header) {
                Ok(_) => {}
                Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(err) => return Err(err.into()),
            }

            let sequence = u64::from_le_bytes(file_header[0..8].try_into().unwrap());
            let length = u32::from_le_bytes(file_header[0..12].try_into().unwrap()) as u64;
            max_sequence = max_sequence.max(sequence);

            let cursor_pos = reader.stream_position()?;
            if cursor_pos + length + 4 > file_length {
                break;
            }
            reader.seek(SeekFrom::Current(length as i64 + 4))?;

        }

        Ok(max_sequence + 1)
    }


    pub fn append() {}
}
