use std::path::Path;

use anyhow::Result;
use sled::Db;

use crate::key::Key;

/// Stores key-url pairs to persistent storage.
///
/// We are using `sled` as an embedded database because it's simple and fast.
/// If we needed more space or we wanted to scale our shortener horizontally, we
/// could use a remote database instead, such as Redis.
#[derive(Clone)]
pub struct Store {
  db: Db,
}

impl Store {
  /// Create a new store.
  pub fn new(path: impl AsRef<Path>) -> Result<Self> {
    Ok(Self {
      db: sled::open(path)?,
    })
  }

  /// Insert a new key-url pair.
  pub fn insert(&self, key: &Key, url: &str) -> Result<()> {
    self.db.insert(key, url)?;
    Ok(())
  }

  /// Get the url for a key.
  pub fn get(&self, key: &Key) -> Result<Option<String>> {
    Ok(
      self
        .db
        .get(key)?
        .map(|v| String::from_utf8_lossy(&v).into_owned()),
    )
  }
}
