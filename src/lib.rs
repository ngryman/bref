//! Bref is a simple URL shortener that generates a unique key for each URL and
//! stores it in a local database.
//!
//! It is designed to be used as as standalone server, but can be used as a
//! library.
//!
//! ## Usage
//!
//! ```rust
//! use bref::Bref;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!   Bref::default().run().await
//! }
//! ```

mod base62;
mod key;
mod server;
mod store;

use std::path::PathBuf;

use anyhow::Result;
use clap::{command, Parser};

use crate::server::Server;

// === Args ===

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
  /// Port to listen to
  #[arg(global = true, long, short, env, default_value_t = 8080)]
  port: u16,

  /// Path to the database
  ///
  /// By default, Bref uses the preferred XDG data directory.
  #[arg(global = true, long, env)]
  db_path: Option<PathBuf>,
}

// === Bref ===

pub struct Bref {
  port: u16,
  db_path: PathBuf,
}

impl Bref {
  /// Run the server.
  pub async fn run(self) -> Result<()> {
    Server::new(self.port, self.db_path).listen().await
  }
}

impl Default for Bref {
  /// Create an instance from the command line arguments.
  fn default() -> Self {
    Self::from(Args::parse())
  }
}

impl From<Args> for Bref {
  /// Convert parsed arguments into an instance.
  ///
  /// This makes sure optional "dynamic" arguments (i.e. `db_path`) are set to
  /// their default values.
  fn from(args: Args) -> Self {
    let db_path = args.db_path.unwrap_or_else(|| {
      xdg::BaseDirectories::with_prefix("bref")
        .unwrap()
        .get_data_dirs()
        .first()
        .unwrap()
        .to_owned()
    });

    Self {
      port: args.port,
      db_path,
    }
  }
}
