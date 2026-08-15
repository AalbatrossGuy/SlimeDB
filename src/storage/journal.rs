use crate::types::{SlimeDBError, Result};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

pub struct Journal {
    file: File,
}
