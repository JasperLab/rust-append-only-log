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
    offset: usize,
}

#[allow(unused_variables)]
impl Log {
    pub fn new() -> Log {
        Log {
            records: Mutex::new(vec![]),
        }
    }

    pub fn append(&mut self, mut record: Record) -> Result<usize, LogError> {
        let mut guard = self.records.lock().unwrap();
        let offset = guard.len();
        record.offset = offset;
        guard.push(record);
        Ok(offset)
    }

    pub fn read(&mut self, offset: usize) -> Result<Record, LogError> {
        Ok(Record {
            offset: offset,
            value: vec![],
        })
    }
}

impl Record {
    pub fn new(value: Vec<u8>) -> Record {
        let offset = 0;
        Record { value, offset }
    }
}

#[test]
fn test_append() {
    let mut log = Log::new();
    let record = Record::new(vec![127]);
    assert_eq!(log.append(record).unwrap(), 0)
}
