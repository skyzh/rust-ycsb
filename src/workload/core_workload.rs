use crate::generator::{AcknowledgedCounterGenerator, DiscreteGenerator, Generators};

#[derive(Copy, Clone, Debug)]
pub enum CoreOperation {
    Read,
    Update,
    Insert,
    Scan,
    ReadModifyWrite,
}

pub struct CoreWorkload {
    table: String,
    field_count: u64,
    field_names: Vec<String>,
    field_length_generator: Generators,
    read_all_fields: bool,
    write_all_fields: bool,
    data_integrity: bool,
    key_sequence: Generators,
    operation_chooser: DiscreteGenerator<CoreOperation>,
    key_chooser: Generators,
    field_chooser: Generators,
    transaction_insert_key_sequence: AcknowledgedCounterGenerator,
}
