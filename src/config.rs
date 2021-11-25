//! Deserialize TOML config files.

use std::collections::HashMap;
use std::fs;
use std::io::Read;

use serde::Deserialize;

use crate::util;

/// Representation of a `tdb` config file.
#[derive(Debug, Deserialize)]
pub struct Config<'a> {
    #[serde(borrow, rename = "Servers")]
    pub servers: Servers<'a>,
    #[serde(borrow, rename = "Staff")]
    pub staff: Staff<'a>,
    #[serde(borrow, rename = "StaffBadges")]
    pub staff_badges: StaffBadges<'a>,
}

/// Representation of a `tdb` config file `Servers` object.
pub type Servers<'a> = HashMap<&'a str, ServerInfo<'a>>;

/// Representation of a `tdb` config file `ServerInfo` object.
#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum ServerInfo<'a> {
    Tuple(&'a str),
    Struct {
        url: &'a str,
        #[serde(default = "default_port")]
        port: u16,
    },
}

/// The default port for SQL Server.
const fn default_port() -> u16 {
    1433
}

/// Representation of a `tdb` config file `Staff` object.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Staff<'a> {
    #[serde(rename = "LoginUserId")]
    pub login_user_id: &'a str,
    #[serde(rename = "PIN")]
    pub pin: &'a str,
    #[serde(rename = "FirstName")]
    pub first_name: &'a str,
    #[serde(rename = "LastName")]
    pub last_name: &'a str,
    #[serde(rename = "NTUserName")]
    pub nt_username: &'a str,
    #[serde(rename = "EmailAddress")]
    pub email_address: &'a str,
    #[serde(rename = "SSOUserId")]
    pub sso_user_id: &'a str,
}

/// Representation of a `tdb` config file `StaffBadges` object.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StaffBadges<'a> {
    #[serde(rename = "LoginUserId")]
    pub login_user_id: Option<&'a str>,
    #[serde(rename = "BadgeData")]
    pub badge_data: &'a str,
}

/// Load a config file from the current directory into a buffer and return the deserialized
/// [`Config`].
#[tracing::instrument(name = "config", level = "debug", skip_all, fields(file = file_name))]
pub fn load<'a>(file_name: &str, buf: &'a mut String) -> util::Result<Config<'a>> {
    let mut file = fs::File::open(file_name)?;
    let len = file.read_to_string(buf)?;
    let cfg: Config = toml::from_str(buf)?;

    tracing::debug!("buf.capacity()={}", buf.capacity());
    tracing::debug!("file.len()={}", len);
    tracing::trace!(?cfg);
    tracing::info!("{} loaded", file_name);

    Ok(cfg)
}
