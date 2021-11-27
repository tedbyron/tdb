//! Database operations.

use std::time::Duration;

use clap::ArgMatches;
#[cfg(windows)]
use tiberius::AuthMethod;
use tiberius::{Client, EncryptionLevel};
use tokio::net::TcpStream;
use tokio::time;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

use crate::config::ServerInfo;
use crate::util;

/// Dispatch a database query to an address.
#[tracing::instrument(level = "debug", skip_all)]
pub async fn dispatch(info: ServerInfo<'_>, matches: &ArgMatches) -> util::Result<()> {
    // Parse the database name. Unwrap is safe because the arg is required.
    let db_arg = matches.value_of("DATABASE").unwrap();
    tracing::debug!(db_arg);
    let db = parse_db_name(db_arg);
    tracing::info!(%db);

    // Get the operation name. Unwrap is safe because the arg is required.
    let operation = matches.value_of("OPERATION").unwrap();
    tracing::info!(operation);

    // Get the table name. Unwrap is safe because the arg is required.
    let table = matches.value_of("TABLE").unwrap();
    tracing::info!(table);

    match operation {
        "s" | "select" => select(info, &db, table, matches).await?,
        "i" | "insert" => insert(info, &db, table, matches).await?,
        "u" | "update" => update(info, &db, table, matches).await?,
        _ => unreachable!(), // CLI already checked the value.
    }

    Ok(())
}

/// Check if a database name is a database code (3 ASCII characters and 2 ASCII digits).
fn is_db_code(db_name: &str) -> bool {
    if db_name.len() != 5 {
        return false;
    }

    db_name[0..3].chars().all(|c| c.is_ascii_alphabetic())
        && db_name[3..5].chars().all(|c| c.is_ascii_digit())
}

/// Parse a database name, expanding the name if it is only a database code.
fn parse_db_name(db_name: &str) -> String {
    if is_db_code(db_name) {
        return String::from("acgapplication_") + db_name;
    }

    db_name.to_owned()
}

/// Create a `tiberius::Config` for the database client.
#[tracing::instrument(level = "debug", skip_all)]
fn config(info: ServerInfo<'_>, db: &str) -> tiberius::Config {
    let mut cfg = tiberius::Config::new();

    cfg.application_name("tdb");
    #[cfg(windows)]
    cfg.authentication(AuthMethod::Integrated);
    cfg.encryption(EncryptionLevel::Required);
    cfg.host(info.url());
    cfg.port(info.port());
    cfg.database(db);

    tracing::trace!(?cfg);

    cfg
}

/// Create a `tiberius::Client` with a TCP connection to a database server.
#[tracing::instrument(level = "debug", skip_all)]
async fn build_client(info: ServerInfo<'_>, db: &str) -> util::Result<Client<Compat<TcpStream>>> {
    let cfg = config(info, db);
    let tcp = time::timeout(Duration::from_secs(3), TcpStream::connect(cfg.get_addr())).await??;
    tracing::info!("connected to {}", cfg.get_addr());

    // Buffering is handled internally with a `Sink`.
    tcp.set_nodelay(true)?;
    tracing::trace!(?tcp);
    let client = Client::connect(cfg, tcp.compat_write()).await?;
    tracing::trace!(?client);

    Ok(client)
}

/// Execute a SELECT statement.
#[tracing::instrument(level = "debug", skip_all)]
async fn select(
    info: ServerInfo<'_>,
    db: &str,
    _table: &str,
    _matches: &ArgMatches,
) -> util::Result<()> {
    let _client = build_client(info, db).await?;
    Ok(())
}

/// Execute an INSERT statement.
#[tracing::instrument(level = "debug", skip_all)]
async fn insert(
    _info: ServerInfo<'_>,
    _db: &str,
    _table: &str,
    _matches: &ArgMatches,
) -> util::Result<()> {
    Ok(())
}

/// Execute an UPDATE statement.
#[tracing::instrument(level = "debug", skip_all)]
async fn update(
    _info: ServerInfo<'_>,
    _db: &str,
    _table: &str,
    _matches: &ArgMatches,
) -> util::Result<()> {
    Ok(())
}
