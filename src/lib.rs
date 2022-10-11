use std::sync::Arc;
use std::sync::RwLock;
use thiserror::Error;

/// Implements the append-only log data structure.
/// This implementation takes the ownership of the stored data
/// and returns the atomic reference count to it when reading.
/// The data survives past the Log instance destruction.
pub struct Log {
    /// RwLock-protected list of record atomic references
    records: RwLock<Vec<Arc<Record>>>,
}

#[derive(Error, Debug)]
#[error("{message}")]
pub struct LogError {
    message: String,
}

pub struct Record {
    value: Vec<u8>,
    offset: usize,
}

impl Log {
    pub fn new() -> Log {
        Log {
            records: RwLock::new(vec![]),
        }
    }

    pub fn append(&self, mut record: Record) -> Result<usize, LogError> {
        let mut guard = self.records.write().unwrap();
        let offset = guard.len();
        record.offset = offset;
        guard.push(Arc::new(record));
        Ok(offset)
    }

    pub fn read(&self, offset: usize) -> Result<Arc<Record>, LogError> {
        let guard = self.records.read().unwrap();
        if offset >= guard.len() {
            Err(LogError::new("Provided offset is invalid"))
        } else {
            let r = guard[offset].clone();
            Ok(r)
        }
    }
}

impl Record {
    pub fn new(value: Vec<u8>) -> Record {
        let offset = 0;
        Record { value, offset }
    }
}

impl LogError {
    pub fn new(msg: &str) -> LogError {
        LogError {
            message: msg.to_string(),
        }
    }
}

#[test]
fn test_append() {
    let log = Log::new();
    let record = Record::new(vec![127]);
    assert_eq!(log.append(record).unwrap(), 0)
}

#[test]
fn test_read() {
    let log = Log::new();
    let record = Record::new(vec![127]);
    let offset = log.append(record).unwrap();
    let r = log.read(offset).unwrap();
    assert_eq!(r.value, vec![127]);
}
