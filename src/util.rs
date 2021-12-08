//! Utility items.

use std::borrow::Cow;
use std::process;

use tiberius::numeric::Numeric;
use tiberius::time::chrono;
// use tiberius::xml::XmlData;
use tiberius::{ColumnData, FromSql as _};

/// A `Result` type that may contain a thread-safe, static `Error`.
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

pub trait ColumnString {
    fn to_string(&self) -> String;
}

impl ColumnString for ColumnData<'static> {
    fn to_string(&self) -> String {
        match self {
            ColumnData::U8(n) => n.unwrap_or_default().to_string(),
            ColumnData::I16(n) => n.unwrap_or_default().to_string(),
            ColumnData::I32(n) => n.unwrap_or_default().to_string(),
            ColumnData::I64(n) => n.unwrap_or_default().to_string(),
            ColumnData::F32(n) => n.unwrap_or_default().to_string(),
            ColumnData::F64(n) => n.unwrap_or_default().to_string(),
            ColumnData::Bit(b) => b.unwrap_or_default().to_string(),
            ColumnData::String(s) => s.as_ref().unwrap_or(&Cow::Borrowed("")).to_string(),
            ColumnData::Guid(guid) => guid.unwrap_or_default().to_hyphenated().to_string(),
            ColumnData::Binary(b) => {
                format!("{:?}", b.as_ref().unwrap_or_else(|| &Cow::Borrowed(&[])))
            }
            ColumnData::Numeric(n) => {
                format!("{:?}", n.unwrap_or_else(|| Numeric::new_with_scale(0, 0)))
            }
            ColumnData::Xml(_xml) => format!(
                "{:?}",
                1 // FIX
                  // xml.as_ref()
                  //     .unwrap_or_else(|| &Cow::Borrowed(&XmlData::new("")))
            ),
            datetime @ ColumnData::DateTime(_) => format!(
                "{}",
                chrono::NaiveDateTime::from_sql(datetime)
                    .unwrap()
                    .unwrap_or_else(|| chrono::NaiveDateTime::from_timestamp(0, 0))
            ),
            datetime @ ColumnData::SmallDateTime(_) => format!(
                "{}",
                chrono::NaiveDateTime::from_sql(datetime)
                    .unwrap()
                    .unwrap_or_else(|| chrono::NaiveDateTime::from_timestamp(0, 0))
            ),
            time @ ColumnData::Time(_) => format!(
                "{}",
                chrono::NaiveTime::from_sql(time)
                    .unwrap()
                    .unwrap_or_else(|| chrono::NaiveTime::from_hms(0, 0, 0))
            ),
            date @ ColumnData::Date(_) => format!(
                "{}",
                chrono::NaiveDate::from_sql(date)
                    .unwrap()
                    .unwrap_or_else(|| chrono::NaiveDate::from_ymd(1970, 1, 1))
            ),
            datetime @ ColumnData::DateTime2(_) => format!(
                "{}",
                chrono::NaiveDateTime::from_sql(datetime)
                    .unwrap()
                    .unwrap_or_else(|| chrono::NaiveDateTime::from_timestamp(0, 0))
            ),
            datetimeoffset @ ColumnData::DateTimeOffset(_) => format!(
                "{}",
                chrono::DateTime::from_sql(datetimeoffset)
                    .unwrap()
                    .unwrap_or_else(|| chrono::DateTime::<chrono::Utc>::from_utc(
                        chrono::NaiveDateTime::from_timestamp(0, 0),
                        chrono::Utc
                    ))
            ),
        }
    }
}
