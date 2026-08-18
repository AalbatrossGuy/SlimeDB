// Created by AG on 13-08-2026

use std::time::{self, SystemTime, UNIX_EPOCH};
use std::{fmt, sync::mpsc::Receiver};
use thiserror::Error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type Timestamp = u64;
pub type TransactionId = u64;
pub type BranchId = Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Key(pub Vec<u8>);

impl Key {
    pub fn new(data: impl Into<Vec<u8>>) -> Self {
        Self(data.into())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl From<&str> for Key {
    fn from(string: &str) -> Self {
        Self(string.as_bytes().to_vec())
    }
}

impl From<String> for Key {
    fn from(string: String) -> Self {
        Self(string.into_bytes())
    }
}

impl fmt::Display for Key {
    fn fmt(&self, frmtr: &mut fmt::Formatter<'_>) -> fmt::Result {
        match std::str::from_utf8(&self.0) {
            Ok(string) => write!(frmtr, "{}", string),
            Err(_) => write!(frmtr, "{:?}", self.0),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Value(pub Vec<u8>);

impl Value {
    pub fn new(data: impl Into<Vec<u8>>) -> Self {
        Self(data.into())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn length(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<&str> for Value {
    fn from(string: &str) -> Self {
        Self(string.as_bytes().to_vec())
    }
}

impl From<String> for Value {
    fn from (string: String) -> Self {
        Self(string.into_bytes())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DBRecordType {
    Put,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DBRecordVersioned {
    pub key: Key,
    pub value: Option<Value>,
    pub timestamp: Timestamp,
    pub transaction_id: TransactionId,
    pub branch_id: BranchId,
    pub db_record_type: DBRecordType,
}

impl DBRecordVersioned {
    pub fn put(key: Key, value: Value, timestamp: Timestamp, transaction_id: TransactionId, branch_id: BranchId) -> Self {
        Self {
            key,
            value: Some(value),
            timestamp,
            transaction_id,
            branch_id,
            db_record_type: DBRecordType::Put,
        }
    }

    pub fn delete(key: Key, value: Value, timestamp: Timestamp, transaction_id: TransactionId, branch_id: BranchId) -> Self {
        Self {
            key,
            value: None,
            timestamp,
            transaction_id,
            branch_id,
            db_record_type: DBRecordType::Delete,
        }
    }

    pub fn is_delete_marker(&self) -> bool {
        self.db_record_type == DBRecordType::Delete
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DBBranch {
    pub branch_id: BranchId,
    pub branch_name: String,
    pub branch_parent_id: Option<BranchId>,
    pub fork_timestamp: Timestamp,
    pub branch_created_at: Timestamp,
}

impl DBBranch {
    pub fn main() -> Self {
        Self {
            branch_id: Uuid::nil(),
            branch_name: "main".to_string(),
            branch_parent_id: None,
            fork_timestamp: 0,
            branch_created_at: 0,
        }
    }

    pub fn new(branch_name: String, branch_parent: &DBBranch, fork_timestamp: Timestamp) -> Self {
        Self {
            branch_id: Uuid::new_v4(),
            branch_name: branch_name,
            branch_parent_id: Some(branch_parent.branch_id),
            fork_timestamp: fork_timestamp,
            branch_created_at: current_timestamp(),
        }
    }
}

pub fn current_timestamp() -> Timestamp {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as Timestamp
}

#[derive(Error, Debug)]
pub enum SlimeDBError {
    #[error("[SlimeDB] I/O Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("[SlimeDB] Data Corruption Detected: {0}")]
    Corruption(String),

    #[error("[SlimeDB] Key Not Found: {0}")]
    KeyNotFound(String),

    #[error("[SlimeDB] Serialization Error: {0}")]
    Serialization(String),
}

pub type Result<T> = std::result::Result<T, SlimeDBError>;
