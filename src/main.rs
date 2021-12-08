#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    rust_2018_idioms
)]
#![allow(clippy::too_many_lines)]
#![doc = include_str!("../README.md")]
#![windows_subsystem = "console"]

mod config;
mod db;
mod util;

use std::env;

use clap::{App, Arg};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::time;
use tracing_subscriber::EnvFilter;

use crate::util::OkOrExit;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // All errors are propagated here so that destructors are called before the process exits with
    // an error code.
    run().await.or_exit(1);
    tracing::info!("Done");
}

async fn run() -> util::Result<()> {
    // Default to tracing `LEVEL::WARN`.
    if env::var("TDB_LOG").is_err() {
        env::set_var("TDB_LOG", "WARN");
    }
    if env::args().any(|a| a == "--info") {
        env::set_var("TDB_LOG", "INFO");
    }
    if env::args().any(|a| a == "--trace") {
        env::set_var("TDB_LOG", "TRACE");
    }

    // Global tracing subscriber.
    let sub = tracing_subscriber::fmt()
        .with_target(false) // Don't show source file names.
        .with_timer(time::uptime()) // Use program uptime instead of system time.
        .with_span_events(FmtSpan::CLOSE) // Show elapsed time on span exit.
        .with_env_filter(EnvFilter::from_env("TDB_LOG")); // Filter using $TDB_LOG.
    #[cfg(feature = "ansi")]
    let sub = sub.with_ansi(true); // Support color for Windows terminals.
    sub.init();

    // https://github.com/rust-lang/rust/issues/79524
    tracing::trace!(command = %env::args().collect::<Vec<_>>().join(" "));

    // Load the config file into a buffer and deserialize it.
    let mut buf = String::with_capacity(2_048);
    let mut path = env::current_exe()?;
    path.pop();
    path.push("tdb.toml");
    let cfg = config::load(
        path.as_path()
            .to_str()
            .ok_or("path to tdb.toml is not valid unicode")?,
        &mut buf,
    )?;

    // Create the CLI app.
    let span = tracing::trace_span!("Build app").entered();
    let mut app = App::new(clap::crate_name!())
        .about(clap::crate_description!())
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .args([
            Arg::new("info")
                .about("Use info output")
                .long("info")
                .conflicts_with("trace"),
            Arg::new("trace")
                .about("Use trace output")
                .long("trace")
                .conflicts_with("info"),
            Arg::new("config")
                .about("Use a custom configuration file")
                .short('c')
                .long("config"),
        ]);

    // Add servers and arguments to the CLI app.
    tracing::trace!("Adding subcommands");
    for (server, info) in &cfg.servers {
        app = app.subcommand(
            App::new(*server).about(info.url()).args([
                // Hidden output level args.
                Arg::new("info")
                    .about("Use info output")
                    .long("info")
                    .conflicts_with("trace")
                    .hidden(true),
                Arg::new("trace")
                    .about("Use trace output")
                    .long("trace")
                    .conflicts_with("info")
                    .hidden(true),
                // Required args.
                Arg::new("DATABASE")
                    .about("The database to use")
                    .takes_value(true)
                    .required(true),
                Arg::new("OPERATION")
                    .about("The operation to perform")
                    .required(true)
                    .takes_value(true)
                    .case_insensitive(true)
                    .possible_values(["s", "select", "i", "insert", "u", "update"]),
                Arg::new("TABLE")
                    .about("The table to operate on")
                    .required(true)
                    .takes_value(true),
                // Optional args.
                Arg::new("SET")
                    .about("A SET clause")
                    .short('s')
                    .long("set")
                    .takes_value(true),
                Arg::new("WHERE")
                    .about("A WHERE clause")
                    .short('w')
                    .long("where")
                    .takes_value(true),
                Arg::new("VALUES")
                    .about("A VALUES clause")
                    .short('v')
                    .long("values")
                    .takes_value(true),
                Arg::new("GROUP_BY")
                    .about("A GROUP BY clause")
                    .short('g')
                    .long("group-by")
                    .takes_value(true),
                Arg::new("ORDER_BY")
                    .about("An ORDER BY clause")
                    .short('o')
                    .long("order-by")
                    .takes_value(true),
            ]),
        );
    }
    drop(span);

    // Parse CLI arguments.
    let span = tracing::trace_span!("parse_args").entered();
    let matches = app.get_matches();
    tracing::trace!(?matches);
    drop(span);

    // Get subcommand matches and perform a database query.
    for (server, info) in cfg.servers {
        if let Some(matches) = matches.subcommand_matches(server) {
            db::dispatch(info, matches).await?;
            break;
        }
    }

    Ok(())
}
