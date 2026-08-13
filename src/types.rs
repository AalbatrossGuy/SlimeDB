// Created by AG on 13-08-2026

use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
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
