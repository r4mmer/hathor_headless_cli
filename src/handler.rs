use crate::params::*;

use std::{collections::HashMap, time::Duration};

use reqwest::{self, Url};
use serde::{Serialize, Deserialize};

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
fn build_headless_url(host: &String, path: &str) -> Result<Url, Box<dyn std::error::Error>> {
    let base_url = Url::parse(&host)?;
    let url = base_url.join(path)?;
    Ok(url)
}

fn build_client(config: &CliConfig) -> reqwest::Result<reqwest::Client> {
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
enum HashMapValue {
    Int(u32),
    String(String),
    Bool(bool),
}

/////////////////////////////////////////// handlers

/// Start a wallet
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_start(params: ParamsStart) -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("seedKey", params.seed_key);
    map.insert("wallet-id", params.wallet_id);

    if let Some(passphrase) = params.passphrase {
        map.insert("passphrase", passphrase);
    }

    let url = build_headless_url(&params.config.host, "/start")?;

    let text_response = build_client(&params.config)?
        .post(url)
        .json(&map)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}

/// Get the configuration string of a token
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_configuration_string(params: ParamsConfigString) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(&params.config.host, "/configuration-string")?;

    let text_response = build_client(&params.config)?
        .get(url)
        .query(&[("token", params.token)])
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}

/// Get the multisig xpubkey of the configured seed
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_multisig_pubkey(params: ParamsMultisigPubkey) -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("seedKey", params.seed_key);

    if let Some(passphrase) = params.passphrase {
        map.insert("passphrase", passphrase);
    }

    let url = build_headless_url(&params.config.host, "/multisig-pubkey")?;

    let text_response = build_client(&params.config)?
        .post(url)
        .json(&map)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}

/// Get the status of a wallet
///
/// # Arguments
///
/// * `params` - Base configuration all cli calls share
/// * `wallet_id` - which wallet to fetch the status
///
pub async fn handle_status(params: CliConfig, wallet_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(&params.host, "/wallet/status")?;

    let text_response = build_client(&params)?
        .get(url)
        .header("X-Wallet-Id", wallet_id)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}

/// Get balance of a token in the given wallet
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_balance(params: ParamsWalletBalance) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(&params.config.host, "/wallet/balance")?;

    let mut req_builder = build_client(&params.config)?
        .get(url)
        .header("X-Wallet-Id", params.wallet_id);

    if let Some(token) = params.token {
        req_builder = req_builder.query(&[("token", token)]);
    }

    let text_response = req_builder
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}

/// Get current address from the given wallet
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_address(params: ParamsWalletAddress) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(&params.config.host, "/wallet/address")?;

    let mut req_builder = build_client(&params.config)?
        .get(url)
        .header("X-Wallet-Id", params.wallet_id);

    if let Some(index) = params.index {
        req_builder = req_builder.query(&[("index", index)]);
    }

    if let Some(mark_as_used) = params.mark_as_used {
        req_builder = req_builder.query(&[("mark_as_used", mark_as_used)]);
    }

    let text_response = req_builder
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}

/// Get the address info if the address belongs to the given wallet
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_address_info(params: ParamsWalletAddressInfo) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(&params.config.host, "/wallet/address-info")?;

    let mut req_builder = build_client(&params.config)?
        .get(url)
        .header("X-Wallet-Id", params.wallet_id)
        .query(&[("address", params.address)]);

    if let Some(token) = params.token {
        req_builder = req_builder.query(&[("token", token)]);
    }

    let text_response = req_builder
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}

/// Get the address index if the address belongs to the given wallet
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_address_index(params: ParamsWalletAddressIndex) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(&params.config.host, "/wallet/address-index")?;

    let text_response = build_client(&params.config)?
        .get(url)
        .header("X-Wallet-Id", params.wallet_id)
        .query(&[("address", params.address)])
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}

/// Get all addresses from the given wallet
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_addresses(params: ParamsWalletAddresses) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(&params.config.host, "/wallet/addresses")?;

    let text_response = build_client(&params.config)?
        .get(url)
        .header("X-Wallet-Id", params.wallet_id)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}

/// Get the transaction history of the given wallet
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_tx_history(params: ParamsWalletTxHistory) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(&params.config.host, "/wallet/tx-history")?;

    let mut req_builder = build_client(&params.config)?
        .get(url)
        .header("X-Wallet-Id", params.wallet_id);

    if let Some(limit) = params.limit {
        req_builder = req_builder.query(&[("limit", limit)]);
    }

    let text_response = req_builder
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}

/// Get the transaction details from the tx_id if the transaction belongs to the wallet.
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_transaction(params: ParamsWalletTransaction) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(&params.config.host, "/wallet/transaction")?;

    let text_response = build_client(&params.config)?
        .get(url)
        .header("X-Wallet-Id", params.wallet_id)
        .query(&[("id", params.id)])
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}

/// Decode the given transaction.
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_decode(params: ParamsWalletDecode) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(&params.config.host, "/wallet/decode")?;

    let mut map = HashMap::new();

    if let Some(tx_hex) = params.tx_hex {
        map.insert("tx_hex", tx_hex);
    }

    if let Some(partial_tx) = params.partial_tx {
        map.insert("partial_tx", partial_tx);
    }

    let text_response = build_client(&params.config)?
        .post(url)
        .header("X-Wallet-Id", params.wallet_id)
        .json(&map)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}

/// Get the number of blocks confirming a given transaction.
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_tx_confirmation(params: ParamsWalletTxConfirmation) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(&params.config.host, "/wallet/tx-confirmation-blocks")?;

    let text_response = build_client(&params.config)?
        .get(url)
        .header("X-Wallet-Id", params.wallet_id)
        .query(&[("id", params.id)])
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}

/// Send a simple transaction.
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_simple_send(params: ParamsWalletSimpleSend) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(&params.config.host, "/wallet/simple-send-tx")?;

    let mut map: HashMap<&str, HashMapValue> = HashMap::new();
    map.insert("address", HashMapValue::String(params.address));
    map.insert("value", HashMapValue::Int(params.value));


    if let Some(change_address) = params.change_address {
        map.insert("change_address", HashMapValue::String(change_address));
    }

    if let Some(token) = params.token {
        map.insert("token", HashMapValue::String(token));
    }

    let text_response = build_client(&params.config)?
        .post(url)
        .header("X-Wallet-Id", params.wallet_id)
        .json(&map)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}

/// Send a transaction as specified in the given body.
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_send(params: ParamsWalletSend) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(&params.config.host, "/wallet/send-tx")?;

    // .header(header::CONTENT_TYPE, "application/json")
    // .body(params.body)
    let text_response = build_client(&params.config)?
        .post(url)
        .header("X-Wallet-Id", params.wallet_id)
        .json(&params.body)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}

/// Create a custom token in the given wallet.
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_create_token(params: ParamsWalletCreateToken) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(&params.config.host, "/wallet/simple-send-tx")?;

    let mut map: HashMap<&str, HashMapValue> = HashMap::new();
    map.insert("name", HashMapValue::String(params.name));
    map.insert("symbol", HashMapValue::String(params.symbol));
    map.insert("amount", HashMapValue::Int(params.amount));

    if let Some(address) = params.address {
        map.insert("address", HashMapValue::String(address));
    }

    if let Some(change_address) = params.change_address {
        map.insert("change_address", HashMapValue::String(change_address));
    }

    if let Some(create_mint) = params.create_mint {
        map.insert("create_mint", HashMapValue::Bool(create_mint));
    }

    if let Some(mint_authority_address) = params.mint_authority_address {
        map.insert("mint_authority_address", HashMapValue::String(mint_authority_address));
    }

    if let Some(allow_external_mint_authority_address) = params.allow_external_mint_authority_address {
        map.insert("allow_external_mint_authority_address", HashMapValue::Bool(allow_external_mint_authority_address));
    }

    if let Some(create_melt) = params.create_melt {
        map.insert("create_melt", HashMapValue::Bool(create_melt));
    }

    if let Some(melt_authority_address) = params.melt_authority_address {
        map.insert("melt_authority_address", HashMapValue::String(melt_authority_address));
    }

    if let Some(allow_external_melt_authority_address) = params.allow_external_melt_authority_address {
        map.insert("allow_external_melt_authority_address", HashMapValue::Bool(allow_external_melt_authority_address));
    }

    let text_response = build_client(&params.config)?
        .post(url)
        .header("X-Wallet-Id", params.wallet_id)
        .json(&map)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}
