//! Utility items.

use std::process;

pub trait OkOrExit<T, E> {
    fn or_exit(self, code: i32) -> T;
}

impl<T, E> OkOrExit<T, E> for Result<T, E>
where
    E: std::fmt::Display,
{
    #[inline]
    fn or_exit(self, code: i32) -> T {
        self.map_err(|error| {
            tracing::error!(%error);
            process::exit(code);
        })
        .unwrap()
    }
}

pub trait SomeOrExit<T> {
    fn or_exit(self, code: i32, val: &str) -> T;
}

impl<T> SomeOrExit<T> for Option<T> {
    #[inline]
    fn or_exit(self, code: i32, val: &str) -> T {
        if self.is_none() {
            tracing::error!("{} did not contain a value", val);
            process::exit(code);
        }
        self.unwrap()
    }
}
