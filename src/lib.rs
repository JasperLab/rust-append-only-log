pub struct Log {
    records: Vec<Record>,
}

pub struct Record {
    value: Vec<u8>,
    offset: u64,
}
