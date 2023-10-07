use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::base62::ToBase62;

#[derive(Debug, Deserialize, Serialize)]
pub struct Key(String);

impl Key {
  /// Generate a new time-based key of 6 characters-length.
  ///
  /// I wanted a short-length key for this project. `Key.gen` produces a
  /// different key every second, which is a decent time resolution for personal
  /// use. I'm definitely not going to shorten multiple links per seconds ;)
  ///
  /// Despite the key being quite short, `Key.gen` guarantees there will not be
  /// collisions over time as time is monotonically increasing. The main
  /// downside is that it doesn't produce a unique key per url, which is
  /// something I can live with.
  ///
  /// Other approaches would involve either 1) using an atomic counter or 2)
  /// using a hash of the url. The counter approach is similar to what we have
  /// here and would produce even shorter urls, but the key would not be of
  /// fixed-length over time, which is something I wanted for consistency. The
  /// hash approach would produce a unique key per url, but the key would be
  /// longer, at least 11 characters (64-bits) if we wanted to avoid
  /// collisions.
  pub fn gen() -> Self {
    Self(
      SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap() // safe: system time is always after the epoch
        .as_secs()
        .to_base62(),
    )
  }
}

impl<T: Hash> From<T> for Key {
  fn from(value: T) -> Self {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    let hash: u64 = hasher.finish();
    Self(hash.to_base62())
  }
}

impl AsRef<[u8]> for Key {
  fn as_ref(&self) -> &[u8] {
    self.0.as_bytes()
  }
}
