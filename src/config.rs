//! Deserialize TOML config files.

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crate::util;

/// A `tdb` config file.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config<'cfg> {
    #[serde(borrow)]
    pub servers: Servers<'cfg>,
    #[serde(borrow)]
    pub staff: Staff<'cfg>,
    #[serde(borrow)]
    pub staff_badges: StaffBadges<'cfg>,
}

/// A `tdb` config file `Servers` object.
pub type Servers<'cfg> = HashMap<&'cfg str, ServerInfo<'cfg>>;

/// A `tdb` config file `ServerInfo` object.
#[derive(Debug, Clone, Copy, serde::Deserialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum ServerInfo<'cfg> {
    Tuple(&'cfg str),
    Struct {
        url: &'cfg str,
        #[serde(default = "default_port")]
        port: u16,
    },
}

/// The default port for SQL Server (1433).
const fn default_port() -> u16 {
    1433
}

/// A `tdb` config file `Staff` object.
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct Staff<'cfg> {
    pub login_user_id: &'cfg str,
    #[serde(rename = "PIN")]
    pub pin: &'cfg str,
    pub first_name: &'cfg str,
    pub last_name: &'cfg str,
    #[serde(rename = "NTUserName")]
    pub nt_username: &'cfg str,
    pub email_address: &'cfg str,
    #[serde(rename = "SSOUserId")]
    pub sso_user_id: &'cfg str,
}

/// A `tdb` config file `StaffBadges` object.
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct StaffBadges<'cfg> {
    pub login_user_id: Option<&'cfg str>,
    pub badge_data: &'cfg str,
}

/// Load a config file from the current directory into a buffer and return the deserialized
/// [`Config`]. The buffer's capacity should be greater than or equal to the file's contents to
/// avoid reallocations.
#[tracing::instrument(name = "config", level = "debug", skip_all, fields(file = file_name))]
pub fn load<'a>(file_name: &str, buf: &'a mut String) -> util::Result<Config<'a>> {
    let mut file = File::open(file_name)?;
    let len = file.read_to_string(buf)?;
    let cfg: Config = toml::from_str(buf)?;

    tracing::debug!("buf.capacity()={}", buf.capacity());
    tracing::debug!("file.len()={}", len);
    tracing::trace!(?cfg);
    tracing::info!("{} loaded", file_name);

    Ok(cfg)
}
