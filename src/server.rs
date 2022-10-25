use crate::log::{Log, Record};

struct ProduceRequest {
    record: Record,
}

struct ProduceResponse {
    offset: u64,
}

struct ConsumeRequest {
    offset: u64,
}

struct ConsumeResponse {
    record: Record,
}
