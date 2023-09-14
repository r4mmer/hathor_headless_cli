use crate::data::*;
use crate::params::CliConfig;
use crate::utils::*;

use reqwest::Response;

pub async fn get_address_info(
    config: CliConfig,
    wallet_id: String,
    address: String,
    token: Option<String>,
) -> Result<Response, Box<dyn std::error::Error>> {
    let url = build_headless_url(&config.host, "/wallet/address-info")?;

    let mut req_builder = build_client(&config)?
        .get(url)
        .header("X-Wallet-Id", wallet_id)
        .query(&[("address", address)]);

    if let Some(token) = token {
        req_builder = req_builder.query(&[("token", token)]);
    }

    let response = req_builder.send().await?;
    Ok(response)
}

pub async fn is_address_mine(
    config: CliConfig,
    wallet_id: String,
    address: String,
) -> Result<bool, Box<dyn std::error::Error>> {
    let response = get_address_info(config, wallet_id, address, None)
        .await?
        .json::<AddressInfoResponse>()
        .await?;

    return Ok(response.success);
}
