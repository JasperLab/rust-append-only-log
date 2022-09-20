#[allow(dead_code)]
pub struct Log {
    records: Vec<Record>,
}

#[allow(dead_code)]
pub struct Record {
    value: Vec<u8>,
    offset: u64,
}
