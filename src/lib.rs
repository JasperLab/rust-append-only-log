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
#[derive(Clone)]
pub struct Record {
    value: Vec<u8>,
    offset: usize,
}

#[allow(unused_variables)]
impl Log {
    pub fn new() -> Log {
        Log {
            records: Mutex::new(vec![]),
        }
    }

    pub fn append(&self, record: &Record) -> Result<usize, LogError> {
        let mut guard = self.records.lock().unwrap();
        let offset = guard.len();
        let mut copy = record.clone();
        copy.offset = offset;
        guard.push(copy);
        Ok(offset)
    }

    pub fn read(&self, offset: usize) -> Result<Record, LogError> {
        let guard = self.records.lock().unwrap();
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
    assert_eq!(log.append(&record).unwrap(), 0)
}

#[test]
fn test_read() {
    let log = Log::new();
    let record = Record::new(vec![127]);
    let offset = log.append(&record).unwrap();
    let r = log.read(offset).unwrap();
    assert_eq!(r.value, record.value);
}
