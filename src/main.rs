//! Start the Bref server.
//!
//! ## Usage
//!
//! ```sh
//! $ bref --help
//! Usage: bref [OPTIONS]
//!
//! Options:
//! -p, --port <PORT>
//!         Port to listen to
//!
//!         [env: PORT=]
//!         [default: 8080]
//!
//!     --db-path <DB_PATH>
//!         Path to the database.
//!
//!         By default, Bref uses the preferred XDG data directory.
//!
//!         [env: DB_PATH=]
//!
//! -h, --help
//!         Print help (see a summary with '-h')
//!
//! -V, --version
//!         Print version
//! ```

use anyhow::Result;
use bref::Bref;

#[tokio::main]
async fn main() -> Result<()> {
  Bref::default().run().await
}
