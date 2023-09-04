use crate::params::*;

use std::collections::HashMap;

use reqwest::{self, Url};
use serde::{Serialize, Deserialize};

/////////////////////////////////////////// Utils

fn build_headless_url(host: String, path: &str) -> Result<Url, Box<dyn std::error::Error>> {
    let base_url = Url::parse(&host)?;
    let url = base_url.join(path)?;
    return Ok(url);
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum HashMapValue {
    Int(u32),
    String(String),
    Bool(bool),
}

/////////////////////////////////////////// handlers

pub async fn handle_start(params: ParamsStart) -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("seedKey", params.seed_key);
    map.insert("wallet-id", params.wallet_id);

    match params.passphrase {
        Some(passphrase) => {
            map.insert("passphrase", passphrase);
        }
        None => {}
    }

    let url = build_headless_url(params.config.host, "/start")?;

    let text_response = reqwest::Client::new()
        .post(url)
        .json(&map)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}

pub async fn handle_configuration_string(params: ParamsConfigString) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(params.config.host, "/configuration-string")?;

    let text_response = reqwest::Client::new()
        .get(url)
        .query(&[("token", params.token)])
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}

pub async fn handle_multisig_pubkey(params: ParamsMultisigPubkey) -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("seedKey", params.seed_key);

    match params.passphrase {
        Some(passphrase) => {
            map.insert("passphrase", passphrase);
        }
        None => {}
    }

    let url = build_headless_url(params.config.host, "/multisig-pubkey")?;

    let text_response = reqwest::Client::new()
        .post(url)
        .json(&map)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}

pub async fn handle_status(params: CliConfig, wallet_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(params.host, "/wallet/status")?;

    let text_response = reqwest::Client::new()
        .get(url)
        .header("X-Wallet-Id", wallet_id)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}

pub async fn handle_balance(params: ParamsWalletBalance) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(params.config.host, "/wallet/balance")?;

    let mut req_builder = reqwest::Client::new()
        .get(url)
        .header("X-Wallet-Id", params.wallet_id);

    match params.token {
        Some(token) => {
            req_builder = req_builder.query(&[("token", token)]);
        }
        None => {}
    }

    let text_response = req_builder
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}

pub async fn handle_address(params: ParamsWalletAddress) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(params.config.host, "/wallet/address")?;

    let mut req_builder = reqwest::Client::new()
        .get(url)
        .header("X-Wallet-Id", params.wallet_id);

    match params.index {
        Some(index) => {
            req_builder = req_builder.query(&[("index", index)]);
        }
        None => {}
    }

    match params.mark_as_used {
        Some(mark_as_used) => {
            req_builder = req_builder.query(&[("mark_as_used", mark_as_used)]);
        }
        None => {}
    }

    let text_response = req_builder
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}

pub async fn handle_address_info(params: ParamsWalletAddressInfo) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(params.config.host, "/wallet/address-info")?;

    let mut req_builder = reqwest::Client::new()
        .get(url)
        .header("X-Wallet-Id", params.wallet_id)
        .query(&[("address", params.address)]);

    match params.token {
        Some(token) => {
            req_builder = req_builder.query(&[("token", token)]);
        }
        None => {}
    }

    let text_response = req_builder
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}

pub async fn handle_address_index(params: ParamsWalletAddressIndex) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(params.config.host, "/wallet/address-index")?;

    let text_response = reqwest::Client::new()
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

pub async fn handle_addresses(params: ParamsWalletAddresses) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(params.config.host, "/wallet/addresses")?;

    let text_response = reqwest::Client::new()
        .get(url)
        .header("X-Wallet-Id", params.wallet_id)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}

pub async fn handle_tx_history(params: ParamsWalletTxHistory) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(params.config.host, "/wallet/tx-history")?;

    let mut req_builder = reqwest::Client::new()
        .get(url)
        .header("X-Wallet-Id", params.wallet_id);

    match params.limit {
        Some(limit) => {
            req_builder = req_builder.query(&[("limit", limit)]);
        }
        None => {}
    }

    let text_response = req_builder
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}

pub async fn handle_transaction(params: ParamsWalletTransaction) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(params.config.host, "/wallet/transaction")?;

    let text_response = reqwest::Client::new()
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

pub async fn handle_decode(params: ParamsWalletDecode) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(params.config.host, "/wallet/decode")?;

    let mut map = HashMap::new();

    match params.tx_hex {
        Some(tx_hex) => {
            map.insert("tx_hex", tx_hex);
        }
        None => {}
    }

    match params.partial_tx {
        Some(partial_tx) => {
            map.insert("partial_tx", partial_tx);
        }
        None => {}
    }

    let text_response = reqwest::Client::new()
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

pub async fn handle_tx_confirmation(params: ParamsWalletTxConfirmation) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(params.config.host, "/wallet/tx-confirmation-blocks")?;

    let text_response = reqwest::Client::new()
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

pub async fn handle_simple_send(params: ParamsWalletSimpleSend) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(params.config.host, "/wallet/simple-send-tx")?;

    let mut map: HashMap<&str, HashMapValue> = HashMap::new();
    map.insert("address", HashMapValue::String(params.address));
    map.insert("value", HashMapValue::Int(params.value));


    if let Some(change_address) = params.change_address {
        map.insert("change_address", HashMapValue::String(change_address));
    }

    if let Some(token) = params.token {
        map.insert("token", HashMapValue::String(token));
    }

    let text_response = reqwest::Client::new()
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

pub async fn handle_send(params: ParamsWalletSend) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(params.config.host, "/wallet/send-tx")?;

    // .header(header::CONTENT_TYPE, "application/json")
    // .body(params.body)
    let text_response = reqwest::Client::new()
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

pub async fn handle_create_token(params: ParamsWalletCreateToken) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(params.config.host, "/wallet/simple-send-tx")?;

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

    let text_response = reqwest::Client::new()
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
