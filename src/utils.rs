use std::time::Duration;

use reqwest::{self, Url};
use serde::{Deserialize, Serialize};

use crate::params::*;

/////////////////////////////////////////// Utils

/// Builds the reqwest URL from a base url (a.k.a host) and the required path
/// It may fail if either host or path are not valid.
///
/// # Arguments
///
/// * `host` - Base URL to send the request to
/// * `path` - The path required to be called
///
/// # Examples
///
/// ```
/// let base_url: String = "http://localhost:8000";
/// let actual_url = build_headless_url(&base_url, "/path/to/api")/
/// ```
pub fn build_headless_url(host: &str, path: &str) -> Result<Url, Box<dyn std::error::Error>> {
    let base_url = Url::parse(host)?;
    let url = base_url.join(path)?;
    Ok(url)
}

pub fn build_client(config: &CliConfig) -> reqwest::Result<reqwest::Client> {
    reqwest::Client::builder()
        .connection_verbose(config.debug)
        .connect_timeout(Duration::from_secs(10))
        .user_agent("headless cli")
        .build()
}

/// An enum to wrap the value in a multi-valued HashMap.
/// This allows the HashMap to have string, integers and booleans as value while
/// allowing serializing to json things like:
/// { "address": "H123...", "value": 123, "create_mint": true }
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum HashMapValue {
    Int(u32),
    String(String),
    Bool(bool),
    List(Vec<HashMapValue>),
}
