use std::path::Path;

use ycsb_wrapper::{Database, Result};

pub struct SledWrapper {
    db: sled::Db,
}

impl SledWrapper {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }
}
