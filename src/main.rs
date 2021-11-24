#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![forbid(unsafe_code)]
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

#[tokio::main]
async fn main() {
    // Default to tracing `LEVEL::WARN`.
    if env::var("TDB_LOG").is_err() {
        env::set_var("TDB_LOG", "WARN");
    }
    if env::args().any(|a| a == "--info") {
        env::set_var("TDB_LOG", "INFO");
    }
    if env::args().any(|a| a == "--debug") {
        env::set_var("TDB_LOG", "DEBUG");
    }
    if env::args().any(|a| a == "--trace") {
        env::set_var("TDB_LOG", "TRACE");
    }

    // Tracing subscriber only for the initial config load.
    tracing_subscriber::fmt()
        .with_target(false)
        .with_timer(time::uptime())
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .with_env_filter(EnvFilter::from_env("TDB_LOG"))
        .init();
    let command = env::args().fold(String::with_capacity(50), |s, a| s + &a + " ");
    tracing::debug!(command = command.trim_end());

    // Load the config file into a buffer and deserialize it.
    let mut buf = String::with_capacity(2_048);
    let cfg = config::load("tdb.toml", &mut buf);

    // Create the CLI app.
    let span = tracing::debug_span!("build_app").entered();
    let mut app = App::new(clap::crate_name!())
        .about(clap::crate_description!())
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .args(&[
            Arg::new("debug")
                .about("Use debug level output")
                .long("debug")
                .conflicts_with_all(&["info", "trace"]),
            Arg::new("info")
                .about("Use info level output")
                .long("info")
                .conflicts_with_all(&["debug", "trace"]),
            Arg::new("trace")
                .about("Use trace level output")
                .long("trace")
                .conflicts_with_all(&["debug", "info"]),
            Arg::new("config")
                .about("Use a custom configuration file")
                .short('c')
                .long("config"),
        ]);

    // Add servers and arguments to the CLI app.
    tracing::debug!("adding subcommands");
    for (&server, &info) in &cfg.servers {
        app = app.subcommand(
            App::new(server)
                .about(match info {
                    config::ServerInfo::Tuple(url) | config::ServerInfo::Struct { url, .. } => url,
                })
                .args(&[
                    // Hidden output level args.
                    Arg::new("debug")
                        .about("Use debug level output")
                        .long("debug")
                        .conflicts_with_all(&["info", "trace"])
                        .hidden(true),
                    Arg::new("info")
                        .about("Use info level output")
                        .long("info")
                        .conflicts_with_all(&["debug", "trace"])
                        .hidden(true),
                    Arg::new("trace")
                        .about("Use trace level output")
                        .long("trace")
                        .conflicts_with_all(&["debug", "info"])
                        .hidden(true),
                    // Actual args.
                    Arg::new(db::ARG_NAMES[0])
                        .about("The database to use")
                        .takes_value(true)
                        .required(true)
                        .index(1),
                    Arg::new(db::ARG_NAMES[1])
                        .about("The operation to perform")
                        .required(true)
                        .takes_value(true)
                        .case_insensitive(true)
                        .possible_values(db::OPS)
                        .index(2),
                    Arg::new(db::ARG_NAMES[2])
                        .about("The table to operate on")
                        .required(true)
                        .takes_value(true)
                        .index(3),
                    Arg::new("where")
                        .about("A WHERE clause")
                        .short('w')
                        .long("where")
                        .takes_value(true),
                    Arg::new("set")
                        .about("A SET clause")
                        .short('s')
                        .long("set")
                        .takes_value(true),
                ]),
        );
    }
    drop(span);

    // Parse CLI arguments.
    let span = tracing::debug_span!("parse_args").entered();
    let matches = app.get_matches();
    tracing::trace!(?matches);
    drop(span);

    // Get subcommand matches and perform a database query.
    for (server, info) in cfg.servers {
        if let Some(matches) = matches.subcommand_matches(server) {
            db::dispatch(
                match info {
                    config::ServerInfo::Tuple(url) | config::ServerInfo::Struct { url, .. } => url,
                },
                matches,
            )
            .await;
        }
    }
}
