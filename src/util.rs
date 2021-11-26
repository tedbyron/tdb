//! Utility items.

use std::process;

/// A `Result` type that may contain a thread-safe `Error`.
pub type Result<T> = std::result::Result<T, Box<(dyn std::error::Error + Send + Sync + 'static)>>;

/// Exit the running process if a `Result` contains an `Err`.
pub trait OkOrExit<T, E> {
    /// Exit the running process if a `Result` contains an `Err`. Should only be called from the
    /// top level to ensure that all destructors are called before exiting.
    fn or_exit(self, code: i32) -> T;
}

impl<T, E> OkOrExit<T, E> for std::result::Result<T, E>
where
    E: std::fmt::Display,
{
    fn or_exit(self, code: i32) -> T {
        self.map_err(|error| {
            tracing::error!(%error);
            process::exit(code);
        })
        .unwrap()
    }
}
