use serde::Deserialize;

fn zero_u64() -> u64 {
    0
}

fn thread_count_default() -> u64 {
    200
}

#[derive(Deserialize, Debug)]
pub struct Properties {
    #[serde(default = "zero_u64", rename = "insertstart")]
    insert_start: u64,
    #[serde(rename = "insertcount")]
    insert_count: u64,
    #[serde(rename = "operationcount")]
    operation_count: u64,
    #[serde(default = "zero_u64", rename = "record_count")]
    record_count: u64,
    workload: String,
    db: String,
    exporter: String,
    export_file: String,
    #[serde(default = "thread_count_default", rename = "threacount")]
    thread_count: u64,
    target: String,
    #[serde(rename = "maxexecutiontime")]
    max_execution_time: u64,
    #[serde(rename = "warmuptime")]
    warmup_time: u64,
    #[serde(rename = "dotransactions")]
    do_transactions: bool,
}
