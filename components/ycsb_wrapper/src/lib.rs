use bytes::Bytes;
use std::collections::HashMap;
use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub type FieldMap = HashMap<Bytes, Bytes>;

pub trait Database {
    fn read(&self, table: Bytes, key: Bytes, fields: &[Bytes]) -> Result<FieldMap>;
    fn scan(
        &self,
        table: Bytes,
        start_key: Bytes,
        count: usize,
        fields: &[Bytes],
    ) -> Result<Vec<FieldMap>>;
    fn update(&self, table: Bytes, key: Bytes, values: &[Bytes]) -> Result<()>;
    fn insert(&self, table: Bytes, key: Bytes, values: &[Bytes]) -> Result<()>;
    fn delete(&self, table: Bytes, key: Bytes) -> Result<()>;
}
