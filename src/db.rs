use clap::ArgMatches;

use crate::util::SomeOrExit;

pub const OPS: [&str; 6] = ["s", "select", "i", "insert", "u", "update"];

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum Args {
    DATABASE,
    OP,
    TABLE,
}

impl AsRef<str> for Args {
    fn as_ref(&self) -> &str {
        match self {
            Self::DATABASE => "DATABASE",
            Self::OP => "OP",
            Self::TABLE => "TABLE",
        }
    }
}

/// Dispatch a database query to an address.
#[tracing::instrument(level = "debug", skip(matches))]
pub fn dispatch(address: &str, matches: &ArgMatches) {
    // Parse the database name.
    let db_name = matches
        .value_of(Args::DATABASE.as_ref())
        .or_exit(2, Args::DATABASE.as_ref());
    tracing::debug!(db_name);
    let db_parsed = parse_db_name(db_name);
    tracing::info!(%db_parsed);

    let table_name = matches
        .value_of(Args::TABLE.as_ref())
        .or_exit(2, Args::TABLE.as_ref());
    tracing::info!(table_name);
}

/// Check if a database name is a database code (3 ASCII characters and 2 ASCII digits).
#[inline]
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
