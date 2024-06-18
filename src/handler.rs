use crate::data::*;
use crate::methods::*;
use crate::params::*;
use crate::utils::*;

use std::collections::{HashMap, HashSet};

use log::debug;
use serde_json::json;

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

    if let Some(scan_policy) = params.scan_policy {
        map.insert("scanPolicy", scan_policy);
    }

    if let Some(gap_limit) = params.gap_limit {
        map.insert("gapLimit", gap_limit.to_string());
    }

    if let Some(policy_start_index) = params.policy_start_index {
        map.insert("policyStartIndex", policy_start_index.to_string());
    }

    if let Some(policy_end_index) = params.policy_end_index {
        map.insert("policyEndIndex", policy_end_index.to_string());
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

/// Start an HSM wallet
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_hsm_start(params: ParamsHsmStart) -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("hsm-key", params.hsm_key);
    map.insert("wallet-id", params.wallet_id);

    let url = build_headless_url(&params.config.host, "/hsm/start")?;

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

/// Start a Fireblocks wallet
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_fireblocks_start(params: ParamsFireblocksStart) -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("xpub", params.xpub);
    map.insert("wallet-id", params.wallet_id);

    let url = build_headless_url(&params.config.host, "/fireblocks/start")?;

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
pub async fn handle_configuration_string(
    params: ParamsConfigString,
) -> Result<(), Box<dyn std::error::Error>> {
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
pub async fn handle_multisig_pubkey(
    params: ParamsMultisigPubkey,
) -> Result<(), Box<dyn std::error::Error>> {
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
pub async fn handle_status(
    params: CliConfig,
    wallet_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
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

    let text_response = req_builder.send().await?.text().await?;

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

    let text_response = req_builder.send().await?.text().await?;

    println!("{}", text_response);
    Ok(())
}

/// Get the address info if the address belongs to the given wallet
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_address_info(
    params: ParamsWalletAddressInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(&params.config.host, "/wallet/address-info")?;

    let mut req_builder = build_client(&params.config)?
        .get(url)
        .header("X-Wallet-Id", params.wallet_id)
        .query(&[("address", params.address)]);

    if let Some(token) = params.token {
        req_builder = req_builder.query(&[("token", token)]);
    }

    let text_response = req_builder.send().await?.text().await?;

    println!("{}", text_response);
    Ok(())
}

/// Get the address index if the address belongs to the given wallet
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_address_index(
    params: ParamsWalletAddressIndex,
) -> Result<(), Box<dyn std::error::Error>> {
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
pub async fn handle_addresses(
    params: ParamsWalletAddresses,
) -> Result<(), Box<dyn std::error::Error>> {
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
pub async fn handle_tx_history(
    params: ParamsWalletTxHistory,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(&params.config.host, "/wallet/tx-history")?;

    let mut req_builder = build_client(&params.config)?
        .get(url)
        .header("X-Wallet-Id", params.wallet_id);

    if let Some(limit) = params.limit {
        req_builder = req_builder.query(&[("limit", limit)]);
    }

    let text_response = req_builder.send().await?.text().await?;

    println!("{}", text_response);
    Ok(())
}

/// Get the transaction details from the tx_id if the transaction belongs to the wallet.
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_transaction(
    params: ParamsWalletTransaction,
) -> Result<(), Box<dyn std::error::Error>> {
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
        map.insert("txHex", tx_hex);
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
pub async fn handle_tx_confirmation(
    params: ParamsWalletTxConfirmation,
) -> Result<(), Box<dyn std::error::Error>> {
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
pub async fn handle_simple_send(
    params: ParamsWalletSimpleSend,
) -> Result<(), Box<dyn std::error::Error>> {
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
pub async fn handle_create_token(
    params: ParamsWalletCreateToken,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(&params.config.host, "/wallet/create-token")?;

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
        map.insert(
            "mint_authority_address",
            HashMapValue::String(mint_authority_address),
        );
    }

    if let Some(allow_external_mint_authority_address) =
        params.allow_external_mint_authority_address
    {
        map.insert(
            "allow_external_mint_authority_address",
            HashMapValue::Bool(allow_external_mint_authority_address),
        );
    }

    if let Some(create_melt) = params.create_melt {
        map.insert("create_melt", HashMapValue::Bool(create_melt));
    }

    if let Some(melt_authority_address) = params.melt_authority_address {
        map.insert(
            "melt_authority_address",
            HashMapValue::String(melt_authority_address),
        );
    }

    if let Some(allow_external_melt_authority_address) =
        params.allow_external_melt_authority_address
    {
        map.insert(
            "allow_external_melt_authority_address",
            HashMapValue::Bool(allow_external_melt_authority_address),
        );
    }

    if let Some(data) = params.data
    {
        map.insert("data", HashMapValue::List(data.iter().map(|s| HashMapValue::String(s.clone())).collect()));
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

/// Mint tokens.
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_mint_tokens(
    params: ParamsWalletMintTokens,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(&params.config.host, "/wallet/mint-tokens")?;

    let mut map: HashMap<&str, HashMapValue> = HashMap::new();
    map.insert("token", HashMapValue::String(params.token));
    map.insert("amount", HashMapValue::Int(params.amount));

    if let Some(address) = params.address {
        map.insert("address", HashMapValue::String(address));
    }

    if let Some(change_address) = params.change_address {
        map.insert("change_address", HashMapValue::String(change_address));
    }

    if let Some(mint_authority_address) = params.mint_authority_address {
        map.insert(
            "mint_authority_address",
            HashMapValue::String(mint_authority_address),
        );
    }

    if let Some(allow_external_mint_authority_address) =
        params.allow_external_mint_authority_address
    {
        map.insert(
            "allow_external_mint_authority_address",
            HashMapValue::Bool(allow_external_mint_authority_address),
        );
    }

    if let Some(unshift_data) = params.unshift_data
    {
        map.insert("unshift_data", HashMapValue::Bool(unshift_data));
    }

    if let Some(data) = params.data
    {
        map.insert("data", HashMapValue::List(data.iter().map(|s| HashMapValue::String(s.clone())).collect()));
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

/// Melt tokens
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_melt_tokens(
    params: ParamsWalletMeltTokens,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(&params.config.host, "/wallet/melt-tokens")?;

    let mut map: HashMap<&str, HashMapValue> = HashMap::new();
    map.insert("token", HashMapValue::String(params.token));
    map.insert("amount", HashMapValue::Int(params.amount));

    if let Some(address) = params.address {
        map.insert("address", HashMapValue::String(address));
    }

    if let Some(change_address) = params.change_address {
        map.insert("change_address", HashMapValue::String(change_address));
    }

    if let Some(melt_authority_address) = params.melt_authority_address {
        map.insert(
            "melt_authority_address",
            HashMapValue::String(melt_authority_address),
        );
    }

    if let Some(allow_external_melt_authority_address) =
        params.allow_external_melt_authority_address
    {
        map.insert(
            "allow_external_melt_authority_address",
            HashMapValue::Bool(allow_external_melt_authority_address),
        );
    }

    if let Some(unshift_data) = params.unshift_data
    {
        map.insert("unshiftData", HashMapValue::Bool(unshift_data));
    }

    if let Some(data) = params.data
    {
        map.insert("data", HashMapValue::List(data.iter().map(|s| HashMapValue::String(s.clone())).collect()));
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

/// Get utxos following the rules defined in the given filters
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_utxo_filter(
    params: ParamsWalletUtxoFilter,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(&params.config.host, "/wallet/utxo-filter")?;

    let mut map: HashMap<&str, HashMapValue> = HashMap::new();

    if let Some(max_utxos) = params.max_utxos {
        map.insert("max_utxos", HashMapValue::Int(max_utxos));
    }

    if let Some(token) = params.token {
        map.insert("token", HashMapValue::String(token));
    }

    if let Some(filter_address) = params.filter_address {
        map.insert("filter_address", HashMapValue::String(filter_address));
    }

    if let Some(amount_smaller_than) = params.amount_smaller_than {
        map.insert(
            "amount_smaller_than",
            HashMapValue::Int(amount_smaller_than),
        );
    }

    if let Some(amount_bigger_than) = params.amount_bigger_than {
        map.insert("amount_bigger_than", HashMapValue::Int(amount_bigger_than));
    }

    if let Some(maximum_amount) = params.maximum_amount {
        map.insert("maximum_amount", HashMapValue::Int(maximum_amount));
    }

    if let Some(only_available_utxos) = params.only_available_utxos {
        map.insert(
            "only_available_utxos",
            HashMapValue::Bool(only_available_utxos),
        );
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

/// Consolidate the utxos following the given filters
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_utxo_consolidation(
    params: ParamsWalletUtxoConsolidation,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(&params.config.host, "/wallet/utxo-consolidation")?;

    let mut map: HashMap<&str, HashMapValue> = HashMap::new();

    if let Some(max_utxos) = params.max_utxos {
        map.insert("max_utxos", HashMapValue::Int(max_utxos));
    }

    if let Some(token) = params.token {
        map.insert("token", HashMapValue::String(token));
    }

    if let Some(filter_address) = params.filter_address {
        map.insert("filter_address", HashMapValue::String(filter_address));
    }

    if let Some(amount_smaller_than) = params.amount_smaller_than {
        map.insert(
            "amount_smaller_than",
            HashMapValue::Int(amount_smaller_than),
        );
    }

    if let Some(amount_bigger_than) = params.amount_bigger_than {
        map.insert("amount_bigger_than", HashMapValue::Int(amount_bigger_than));
    }

    if let Some(maximum_amount) = params.maximum_amount {
        map.insert("maximum_amount", HashMapValue::Int(maximum_amount));
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

/// Create an NFT in the given wallet.
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_create_nft(
    params: ParamsWalletCreateNft,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(&params.config.host, "/wallet/create-nft")?;

    let mut map: HashMap<&str, HashMapValue> = HashMap::new();
    map.insert("name", HashMapValue::String(params.name));
    map.insert("symbol", HashMapValue::String(params.symbol));
    map.insert("data", HashMapValue::String(params.data));
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
        map.insert(
            "mint_authority_address",
            HashMapValue::String(mint_authority_address),
        );
    }

    if let Some(allow_external_mint_authority_address) =
        params.allow_external_mint_authority_address
    {
        map.insert(
            "allow_external_mint_authority_address",
            HashMapValue::Bool(allow_external_mint_authority_address),
        );
    }

    if let Some(create_melt) = params.create_melt {
        map.insert("create_melt", HashMapValue::Bool(create_melt));
    }

    if let Some(melt_authority_address) = params.melt_authority_address {
        map.insert(
            "melt_authority_address",
            HashMapValue::String(melt_authority_address),
        );
    }

    if let Some(allow_external_melt_authority_address) =
        params.allow_external_melt_authority_address
    {
        map.insert(
            "allow_external_melt_authority_address",
            HashMapValue::Bool(allow_external_melt_authority_address),
        );
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

/// Stop a wallet.
///
/// # Arguments
///
/// * `params` - arguments to configure the call being made
///
pub async fn handle_stop(params: ParamsWalletStop) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(&params.config.host, "/wallet/stop")?;

    let text_response = build_client(&params.config)?
        .post(url)
        .header("X-Wallet-Id", params.wallet_id)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text_response);
    Ok(())
}

pub async fn handle_list_tokens(
    params: ParamsCustomListTokens,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(&params.config.host, "/wallet/tx-history")?;

    let mut tokens = HashSet::new();

    let tx_history = build_client(&params.config)?
        .get(url)
        .header("X-Wallet-Id", params.wallet_id.clone())
        .send()
        .await?
        .json::<Vec<HistoryTx>>()
        .await?;

    let mut addresses = get_addresses(params.config.clone(), params.wallet_id.clone()).await?;
    let known_addresses: HashSet<String> = addresses.drain(..).collect();

    for tx in tx_history.iter() {
        // Find tokens in the outputs
        for output in tx.outputs.iter() {
            if let Some(address) = output.decoded.address.clone() {
                if known_addresses.contains(&address) {
                    // Address is mine, so the token is mine also
                    tokens.insert(output.token.clone());
                }
            }
        }

        for input in tx.inputs.iter() {
            if let Some(address) = input.decoded.address.clone() {
                if known_addresses.contains(&address) {
                    // Address is mine, so the token is mine also
                    tokens.insert(input.token.clone());
                }
            }
        }
    }

    debug!(
        "Found {} tokens.",
        tokens.len()
    );

    let tokens_json = json!(tokens);
    println!("{}", tokens_json);
    Ok(())
}

pub async fn handle_custom_curl(
    params: ParamsCustomCurl,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_headless_url(&params.config.host, params.path.as_str())?;

    let method = if params.post {
        if params.data {
            " -X POST -d '{}'"
        } else {
            " -X POST"
        }
    } else { "" };
    let mut headers_map = HashMap::<&str, String>::new();
    headers_map.insert("X-Wallet-Id", params.wallet_id);

    if params.post && params.data {
        headers_map.insert("Content-Type", "application/json".to_string());
    }

    let headers = headers_map
        .iter()
        .map(|(k, v)| format!("-H \"{}: {}\"", k, v))
        .collect::<Vec<String>>()
        .join(" ");

    println!("curl{} {} {}", method, headers, url);
    Ok(())
}
