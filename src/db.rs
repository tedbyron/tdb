use clap::ArgMatches;

use crate::util;

pub const ARG_NAMES: [&str; 3] = ["DATABASE", "OP", "TABLE"];
pub const OPS: [&str; 6] = ["s", "select", "i", "insert", "u", "update"];

/// Dispatch a database query to an address.
#[tracing::instrument(level = "debug", skip_all)]
pub async fn dispatch(address: &str, matches: &ArgMatches) -> util::Result<()> {
    // Parse the database name. Unwrap is safe because the arg is required.
    let db_arg = matches.value_of(ARG_NAMES[0]).unwrap();
    tracing::debug!(db_arg);
    let db = parse_db_name(db_arg);
    tracing::info!(%db);

    // Get the operation name. Unwrap is safe because the arg is required.
    let op = matches.value_of(ARG_NAMES[1]).unwrap();
    tracing::info!(op);

    // Get the table name. Unwrap is safe because the arg is required.
    let table = matches.value_of(ARG_NAMES[2]).unwrap();
    tracing::info!(table);

    match op {
        "s" | "select" => select(address, &db, table, matches).await?,
        "i" | "insert" => insert(address, &db, table, matches).await?,
        "u" | "update" => update(address, &db, table, matches).await?,
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

/// Execute a SELECT statement.
#[tracing::instrument(level = "debug", skip_all)]
async fn select(address: &str, db: &str, table: &str, matches: &ArgMatches) -> util::Result<()> {
    Ok(())
}

/// Execute an INSERT statement.
#[tracing::instrument(level = "debug", skip_all)]
async fn insert(address: &str, db: &str, table: &str, matches: &ArgMatches) -> util::Result<()> {
    Ok(())
}

/// Execute an UPDATE statement.
#[tracing::instrument(level = "debug", skip_all)]
async fn update(address: &str, db: &str, table: &str, matches: &ArgMatches) -> util::Result<()> {
    Ok(())
}
