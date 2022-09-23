use std::sync::Mutex;
use thiserror::Error;

#[allow(dead_code)]
pub struct Log {
    records: Mutex<Vec<Record>>,
}

#[derive(Error, Debug)]
#[error("{message}")]
pub struct LogError {
    message: String,
}

#[allow(dead_code)]
pub struct Record {
    value: Vec<u8>,
    offset: u64,
}

#[allow(unused_variables)]
impl Log {
    pub fn append(record: Record) -> Result<u64, LogError> {
        Ok(0)
    }

    pub fn read(offset: u64) -> Result<Record, LogError> {
        Ok(Record {
            offset: offset,
            value: vec![],
        })
    }
}
