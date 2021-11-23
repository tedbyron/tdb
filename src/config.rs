//! Deserialize TOML config files.

use std::collections::HashMap;
use std::fs;
use std::io::Read;

use serde::Deserialize;

use crate::util::OkOrExit;

/// Representation of a `tdb` config file.
#[derive(Debug, Deserialize)]
pub struct Config<'cfg> {
    #[serde(borrow, rename = "Servers")]
    pub servers: HashMap<&'cfg str, &'cfg str>,
    #[serde(borrow, rename = "Staff")]
    pub staff: Staff<'cfg>,
    #[serde(borrow, rename = "StaffBadges")]
    pub staff_badges: StaffBadges<'cfg>,
}

/// Representation of a `tdb` config file `Staff` object.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Staff<'cfg> {
    #[serde(rename = "LoginUserId")]
    pub login_user_id: &'cfg str,
    #[serde(rename = "PIN")]
    pub pin: &'cfg str,
    #[serde(rename = "FirstName")]
    pub first_name: &'cfg str,
    #[serde(rename = "LastName")]
    pub last_name: &'cfg str,
    #[serde(rename = "NTUserName")]
    pub nt_username: &'cfg str,
    #[serde(rename = "EmailAddress")]
    pub email_address: &'cfg str,
    #[serde(rename = "SSOUserId")]
    pub sso_user_id: &'cfg str,
}

/// Representation of a `tdb` config file `StaffBadges` object.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StaffBadges<'cfg> {
    #[serde(rename = "LoginUserId")]
    pub login_user_id: &'cfg str,
    #[serde(rename = "BadgeData")]
    pub badge_data: &'cfg str,
}

/// Load a config file from the current directory into a buffer and return the deserialized
/// [`Config`].
#[tracing::instrument(name = "config", level = "debug", skip(buf))]
pub fn load<'a>(file: &str, buf: &'a mut String) -> Config<'a> {
    let mut file = fs::File::open(file).or_exit(1);
    let len = file.read_to_string(buf).or_exit(1);
    let cfg: Config = toml::from_str(buf).or_exit(1);

    tracing::debug!("buffer capacity: {}", buf.capacity());
    tracing::debug!("file length: {}", len);
    tracing::trace!(?cfg);
    tracing::info!("loaded");

    cfg
}
