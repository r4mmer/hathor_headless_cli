pub mod data;
pub mod handler;
mod methods;
pub mod params;
mod utils;

use handler::*;
use params::*;

use clap::{self, Parser, Subcommand};
// use env_logger;
use std::env;
// use log;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/////////////////////////////////////////// CLI structure

#[derive(Parser)]
#[command(author = "r4mmer", version = VERSION, long_about = None)]
struct Cli {
    #[arg(long, global = true, default_value = "http://localhost:8000")]
    host: String,

    #[arg(
        long,
        global = true,
        default_value_t = false,
        default_missing_value = "true"
    )]
    debug: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a wallet
    Start {
        /// Wallet id to use (all commands for this wallet will require this id)
        #[arg(long, default_value = "default")]
        wallet_id: String,
        /// Key of the seed (on `seeds` in the config)
        #[arg(long, default_value = "default")]
        seed_key: String,
        /// Add this passphrase to the seed (will generate a new wallet with new addresses)
        #[arg(short, long)]
        passphrase: Option<String>,
        /// Use this address scanning policy (defaults to `gap-limit`)
        #[arg(long)]
        scan_policy: Option<String>,
        /// [scan-policy: gap-limit] Always keep `value` addresses without transaction ready.
        #[arg(long)]
        gap_limit: Option<u32>,
        /// [scan-policy: index-limit] Start generating addresses from this address.
        #[arg(long)]
        policy_start_index: Option<u32>,
        /// [scan-policy: index-limit] Stop generating addresses at this address.
        #[arg(long)]
        policy_end_index: Option<u32>,
        /// Use this history sync mode, default is polling_http_api [possible values: polling_http_api, xpub_stream_ws, manual_stream_ws]
        #[arg(long)]
        history_sync_mode: Option<String>,
        /// Start a multisig wallet
        #[arg(long, default_missing_value = "true")]
        multisig: bool,
        /// Key of the multisig config (on `multisig` in the config)
        #[arg(long)]
        multisig_key: Option<String>,
    },

    /// Get the p2sh xpubkey of a configured seed (does not require a started wallet)
    MultisigPubkey {
        /// Which seed to derive the xpubkey
        seed_key: String,
        /// Add this passphrase to the seed
        #[arg(short, long)]
        passphrase: Option<String>,
    },

    /// Fetch the configuration string of a token
    ConfigurationString {
        /// Token UID (hex encoded)
        token: String,
    },

    /// Wallet commands (requires a started wallet)
    Wallet {
        /// Target wallet id
        #[arg(short, long, global = true, default_value = "default")]
        wallet_id: String,

        #[command(subcommand)]
        command: WalletCommands,
    },

    /// Start a Dinamo Networks HSM wallet (requires special configuration)
    Hsm {
        #[command(subcommand)]
        command: HsmCommands,
    },

    /// Start a Fireblocks wallet (requires special configuration)
    Fireblocks {
        #[command(subcommand)]
        command: FireblocksCommands,
    },

    /// Some custom commands and scripts (may require multiple calls)
    Custom {
        #[command(subcommand)]
        command: CustomCommands,
    },
}

#[derive(Subcommand)]
enum CustomCommands {
    /// List all tokens on the wallet history
    ListTokens {
        #[arg(short, long, default_value = "default")]
        wallet_id: String,
    },

    /// Make an http request (simple helper)
    Curl {
        #[arg(short, long, default_value = "default")]
        wallet_id: String,
        #[arg(short, long, default_value_t = false, default_missing_value = "true")]
        post: bool,
        #[arg(short, long, default_value_t = false, default_missing_value = "true")]
        data: bool,
        path: String,
    },
}

#[derive(Subcommand)]
enum HsmCommands {
    /// Start a Dinamo Networks HSM wallet (requires special configuration)
    Start {
        /// HSM key
        hsm_key: String,

        /// Wallet id to use (all commands for this wallet will require this id)
        #[arg(long, default_value = "default")]
        wallet_id: String,
    },
}

#[derive(Subcommand)]
enum FireblocksCommands {
    Start {
        /// xpubkey of the fireblocks wallet
        xpub: String,

        /// Wallet id to use (all commands for this wallet will require this id)
        #[arg(long, default_value = "default")]
        wallet_id: String,
    },
}

#[derive(Subcommand)]
enum WalletCommands {
    /// Wallet balance for a token
    Balance {
        /// Token UID (defaults to 00 [HTR])
        #[arg(short, long)]
        token: Option<String>,
    },

    /// Wallet status
    Status {},

    /// Fetch an address from the wallet
    Address {
        /// Get address on this derivation index
        #[arg(short, long)]
        index: Option<u32>,

        /// Mark address as used (next call will generate another one)
        #[arg(short, long)]
        mark_as_used: Option<bool>,
    },

    /// Get derivation index of an address
    AddressIndex { address: String },

    /// Fetch all wallet addresses
    Addresses {},

    /// Fetch address info (for a token)
    AddressInfo {
        address: String,

        /// Token UID (defaults to 00 [HTR])
        #[arg(short, long)]
        token: Option<String>,
    },

    /// Fetch wallet's tx history
    TxHistory {
        #[arg(short, long)]
        limit: Option<u32>,
    },

    /// Fetch transaction (only if transaction is on the wallet's history)
    Transaction {
        /// Tx ID (hex encoded)
        id: String,
    },

    Decode {
        #[arg(short, long)]
        tx_hex: Option<String>,

        #[arg(short, long)]
        partial_tx: Option<String>,
    },

    /// Get number of blocks confirming this tx.
    TxConfirmation { id: String },

    /// Send a simple transaction
    SimpleSend {
        /// Address (base58 encoded)
        address: String,
        /// Amount of tokens to send
        value: u32,
        #[arg(short, long)]
        change_address: Option<String>,
        /// Token UID (hex encoded)
        #[arg(short, long)]
        token: Option<String>,
    },

    /// Send a complex transaction
    Send {
        /// JSON encoded { outputs: [{address, value, token?, type?, data?, timelock?}, inputs?: {type, index, hash}], change_address }
        body: String,
    },

    /// Send a transaction to create a new token
    CreateToken {
        /// Token name
        name: String,
        /// Token symbol
        symbol: String,
        /// Amount to create
        amount: u32,
        /// Address to send created tokens (base58 encoded)
        #[arg(long)]
        address: Option<String>,
        #[arg(long)]
        change_address: Option<String>,
        /// If a mint authority should be created (default: true)
        #[arg(long)]
        create_mint: Option<bool>,
        /// Send the mint authority to this address (base58 encoded)
        #[arg(long)]
        mint_authority_address: Option<String>,
        /// If we should allow an address not from the wallet as `mint_authority_address`
        #[arg(long)]
        allow_external_mint_authority_address: Option<bool>,
        /// If a melt authority should be created (default: true)
        #[arg(long)]
        create_melt: Option<bool>,
        /// Send the melt authority to this address (base58 encoded)
        #[arg(long)]
        melt_authority_address: Option<String>,
        /// If we should allow an address not from the wallet as `melt_authority_address`
        #[arg(long)]
        allow_external_melt_authority_address: Option<bool>,
        /// List of data outputs to include in the transaction [use multiple times if needed `-d 123 -d 456`]
        #[arg(short, long)]
        data: Option<Vec<String>>,
    },

    /// Send a transaction to mint more tokens (requires ownership of a mint authority output)
    MintTokens {
        /// Token UID (hex encoded)
        token: String,
        /// Amount to mint
        amount: u32,
        /// Address to send new tokens (base58 encoded)
        #[arg(long)]
        address: Option<String>,
        #[arg(long)]
        change_address: Option<String>,
        /// Send the mint authority to this address (base58 encoded)
        #[arg(long)]
        mint_authority_address: Option<String>,
        /// If we should allow an address not from the wallet as `mint_authority_address`
        #[arg(long)]
        allow_external_mint_authority_address: Option<bool>,
        /// Insert data outputs at the beggining of the array
        #[arg(short, long)]
        unshift_data: Option<bool>,
        /// List of data outputs to include in the transaction
        #[arg(short, long)]
        data: Option<Vec<String>>,
    },

    /// Send a transaction to melt tokens (requires ownership of a melt authority output)
    MeltTokens {
        /// Token UID (hex encoded)
        token: String,
        /// Amount to melt
        amount: u32,
        #[arg(long)]
        address: Option<String>,
        #[arg(long)]
        deposit_address: Option<String>,
        #[arg(long)]
        change_address: Option<String>,
        /// Send the melt authority to this address (base58 encoded)
        #[arg(long)]
        melt_authority_address: Option<String>,
        /// If we should allow an address not from the wallet as `melt_authority_address`
        #[arg(long)]
        allow_external_melt_authority_address: Option<bool>,
        /// Insert data outputs at the beggining of the array
        #[arg(short, long)]
        unshift_data: Option<bool>,
        /// List of data outputs to include in the transaction
        #[arg(short, long)]
        data: Option<Vec<String>>,
    },

    /// Find utxos on the wallet that match the filter
    UtxoFilter {
        /// Max number of utxos to fetch.
        #[arg(long)]
        max_utxos: Option<u32>,
        /// Filter by token UID.
        #[arg(long)]
        token: Option<String>,
        /// Filter by address.
        #[arg(long)]
        filter_address: Option<String>,
        /// Only utxos with amount smaller than this.
        #[arg(long)]
        amount_smaller_than: Option<u32>,
        /// Only utxos with amount greater than this.
        #[arg(long)]
        amount_bigger_than: Option<u32>,
        /// The sum of the amounts should not pass this value.
        #[arg(long)]
        maximum_amount: Option<u32>,
        /// Filter by utxos that can be spent right now.
        #[arg(long)]
        only_available_utxos: Option<bool>,
    },

    /// Send a transaction consolidating utxos that match the filter
    UtxoConsolidation {
        /// Max number of utxos to fetch.
        #[arg(long)]
        max_utxos: Option<u32>,
        /// Filter by token UID.
        #[arg(long)]
        token: Option<String>,
        /// Filter by address.
        #[arg(long)]
        filter_address: Option<String>,
        /// Only utxos with amount smaller than this.
        #[arg(long)]
        amount_smaller_than: Option<u32>,
        /// Only utxos with amount greater than this.
        #[arg(long)]
        amount_bigger_than: Option<u32>,
        /// The sum of the amounts should not pass this value.
        #[arg(long)]
        maximum_amount: Option<u32>,
    },

    /// Send a transaction to create a new NFT
    CreateNft {
        /// NFT name.
        name: String,
        /// NFT symbol.
        symbol: String,
        /// Amount to create.
        amount: u32,
        /// NFT data.
        data: String,
        /// Address to send created tokens (base58 encoded).
        #[arg(long)]
        address: Option<String>,
        #[arg(long)]
        change_address: Option<String>,
        /// If a mint authority should be created (default: true)
        #[arg(long)]
        create_mint: Option<bool>,
        /// Send the mint authority to this address (base58 encoded)
        #[arg(long)]
        mint_authority_address: Option<String>,
        /// If we should allow an address not from the wallet as `mint_authority_address`
        #[arg(long)]
        allow_external_mint_authority_address: Option<bool>,
        /// If a melt authority should be created (default: true)
        #[arg(long)]
        create_melt: Option<bool>,
        /// Send the melt authority to this address (base58 encoded)
        #[arg(long)]
        melt_authority_address: Option<String>,
        /// If we should allow an address not from the wallet as `melt_authority_address`
        #[arg(long)]
        allow_external_melt_authority_address: Option<bool>,
    },

    /// Stop a wallet
    Stop {},

    /// Commands for multisig wallets
    P2sh {
        #[command(subcommand)]
        command: P2shTxProposalCommands,
    },
}

#[derive(Subcommand)]
enum P2shTxProposalCommands {
    /// Build a tx proposal
    Build {
        /// JSON encoded: { outputs: [{ address, value, token? }], inputs?: [{tx_id, index}], change_address? }
        body: String,
    },

    CreateToken {
        name: String,
        symbol: String,
        amount: u32,
        #[arg(long)]
        address: Option<String>,
        #[arg(long)]
        change_address: Option<String>,
        #[arg(long)]
        create_mint: Option<bool>,
        #[arg(long)]
        mint_authority_address: Option<String>,
        #[arg(long)]
        allow_external_mint_authority_address: Option<bool>,
        #[arg(long)]
        create_melt: Option<bool>,
        #[arg(long)]
        melt_authority_address: Option<String>,
        #[arg(long)]
        allow_external_melt_authority_address: Option<bool>,
        #[arg(short, long)]
        mark_inputs_as_used: Option<bool>,
    },

    MintTokens {
        token: String,
        amount: u32,
        #[arg(long)]
        address: Option<String>,
        #[arg(long)]
        change_address: Option<String>,
        #[arg(long)]
        create_mint: Option<bool>,
        #[arg(long)]
        mint_authority_address: Option<String>,
        #[arg(long)]
        allow_external_mint_authority_address: Option<bool>,
        #[arg(short, long)]
        mark_inputs_as_used: Option<bool>,
    },
    //
    // MeltTokens {
    //     token: String,
    //     amount: u32,
    //     #[arg(long)]
    //     address: Option<String>,
    //     #[arg(long)]
    //     deposit_address: Option<String>,
    //     #[arg(long)]
    //     change_address: Option<String>,
    //     #[arg(long)]
    //     melt_authority_address: Option<String>,
    //     #[arg(long)]
    //     allow_external_melt_authority_address: Option<bool>,
    //     #[arg(short, long)]
    //     unshift_data: Option<bool>,
    //     #[arg(short, long)]
    //     data: Option<Vec<String>>,
    // },
    /// Get this wallet signatures for a tx proposal
    GetMySignatures { tx_hex: String },

    /// Build signatures and sign proposal
    Sign {
        tx_hex: String,
        signatures: Vec<String>,
    },

    /// Build signatures, sign proposal and push transaction
    SignAndPush {
        tx_hex: String,
        signatures: Vec<String>,
    },
}

async fn handle_custom(
    config: CliConfig,
    custom_cmd: &CustomCommands,
) -> Result<(), Box<dyn std::error::Error>> {
    match custom_cmd {
        CustomCommands::ListTokens { wallet_id } => {
            let params = ParamsCustomListTokens {
                config,
                wallet_id: wallet_id.to_string(),
            };
            handle_list_tokens(params).await?;
        }

        CustomCommands::Curl {
            wallet_id,
            post,
            data,
            path,
        } => {
            let params = ParamsCustomCurl {
                config,
                wallet_id: wallet_id.to_string(),
                post: *post,
                data: *data,
                path: path.to_string(),
            };
            handle_custom_curl(params).await?;
        }
    }

    Ok(())
}

async fn handle_hsm(
    config: CliConfig,
    hsm_cmd: &HsmCommands,
) -> Result<(), Box<dyn std::error::Error>> {
    match hsm_cmd {
        HsmCommands::Start { wallet_id, hsm_key } => {
            let params = ParamsHsmStart {
                config,
                wallet_id: wallet_id.to_string(),
                hsm_key: hsm_key.to_string(),
            };
            handle_hsm_start(params).await?;
        }
    }

    Ok(())
}

async fn handle_fireblocks(
    config: CliConfig,
    fb_cmd: &FireblocksCommands,
) -> Result<(), Box<dyn std::error::Error>> {
    match fb_cmd {
        FireblocksCommands::Start { wallet_id, xpub } => {
            let params = ParamsFireblocksStart {
                config,
                wallet_id: wallet_id.to_string(),
                xpub: xpub.to_string(),
            };
            handle_fireblocks_start(params).await?;
        }
    }

    Ok(())
}

async fn handle_p2sh_txproposal(
    config: CliConfig,
    wallet_id: String,
    command: &P2shTxProposalCommands,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        P2shTxProposalCommands::Build { body } => {
            // handle build
            let params = ParamsP2shTxProposalBuild {
                config,
                wallet_id,
                body: body.to_string(),
            };
            handle_p2sh_txproposal_build(params).await?;
        }

        P2shTxProposalCommands::CreateToken {
            name,
            symbol,
            amount,
            address,
            change_address,
            create_mint,
            mint_authority_address,
            allow_external_mint_authority_address,
            create_melt,
            melt_authority_address,
            allow_external_melt_authority_address,
            mark_inputs_as_used,
        } => {
            let params = ParamsWalletP2shTxProposalCreateToken {
                config,
                wallet_id,
                name: name.to_string(),
                symbol: symbol.to_string(),
                amount: *amount,
                address: address.clone(),
                change_address: change_address.clone(),
                create_mint: *create_mint,
                mint_authority_address: mint_authority_address.clone(),
                allow_external_mint_authority_address: *allow_external_mint_authority_address,
                create_melt: *create_melt,
                melt_authority_address: melt_authority_address.clone(),
                allow_external_melt_authority_address: *allow_external_melt_authority_address,
                mark_inputs_as_used: *mark_inputs_as_used,
            };
            handle_p2sh_txproposal_create_token(params).await?;
        }

        P2shTxProposalCommands::MintTokens {
            token,
            amount,
            address,
            change_address,
            create_mint,
            mint_authority_address,
            allow_external_mint_authority_address,
            mark_inputs_as_used,
        } => {
            let params = ParamsWalletP2shTxProposalMintTokens {
                config,
                wallet_id,
                token: token.to_string(),
                amount: *amount,
                address: address.clone(),
                change_address: change_address.clone(),
                create_mint: *create_mint,
                mint_authority_address: mint_authority_address.clone(),
                allow_external_mint_authority_address: *allow_external_mint_authority_address,
                mark_inputs_as_used: *mark_inputs_as_used,
            };
            handle_p2sh_txproposal_mint_tokens(params).await?;
        }

        P2shTxProposalCommands::GetMySignatures { tx_hex } => {
            let params = ParamsP2shTxProposalGetMySignatures {
                config,
                wallet_id,
                tx_hex: tx_hex.clone(),
            };
            handle_p2sh_txproposal_get_my_signatures(params).await?;
        }

        P2shTxProposalCommands::Sign { tx_hex, signatures } => {
            let params = ParamsP2shTxProposalSign {
                config,
                wallet_id,
                tx_hex: tx_hex.clone(),
                signatures: signatures.clone(),
            };
            handle_p2sh_txproposal_sign(params).await?;
        }

        P2shTxProposalCommands::SignAndPush { tx_hex, signatures } => {
            let params = ParamsP2shTxProposalSign {
                config,
                wallet_id,
                tx_hex: tx_hex.clone(),
                signatures: signatures.clone(),
            };
            handle_p2sh_txproposal_sign_and_push(params).await?;
        }
    }

    Ok(())
}

async fn handle_wallet(
    config: CliConfig,
    wallet_id: String,
    wallet_cmd: &WalletCommands,
) -> Result<(), Box<dyn std::error::Error>> {
    match wallet_cmd {
        WalletCommands::Status {} => {
            handle_status(config, wallet_id.to_string()).await?;
        }

        WalletCommands::Balance { token } => {
            let params = ParamsWalletBalance {
                config,
                wallet_id,
                token: token.clone(),
            };
            handle_balance(params).await?;
        }

        WalletCommands::Address {
            index,
            mark_as_used,
        } => {
            let params = ParamsWalletAddress {
                config,
                wallet_id,
                index: *index,
                mark_as_used: *mark_as_used,
            };
            handle_address(params).await?;
        }

        WalletCommands::AddressInfo { address, token } => {
            let params = ParamsWalletAddressInfo {
                config,
                wallet_id,
                address: address.to_string(),
                token: token.clone(),
            };
            handle_address_info(params).await?;
        }

        WalletCommands::AddressIndex { address } => {
            let params = ParamsWalletAddressIndex {
                config,
                wallet_id,
                address: address.to_string(),
            };
            handle_address_index(params).await?;
        }

        WalletCommands::Addresses {} => {
            let params = ParamsWalletAddresses { config, wallet_id };
            handle_addresses(params).await?;
        }

        WalletCommands::TxHistory { limit } => {
            let params = ParamsWalletTxHistory {
                config,
                wallet_id,
                limit: *limit,
            };
            handle_tx_history(params).await?;
        }

        WalletCommands::Transaction { id } => {
            let params = ParamsWalletTransaction {
                config,
                wallet_id,
                id: id.to_string(),
            };
            handle_transaction(params).await?;
        }

        WalletCommands::Decode { tx_hex, partial_tx } => {
            let params = ParamsWalletDecode {
                config,
                wallet_id,
                tx_hex: tx_hex.clone(),
                partial_tx: partial_tx.clone(),
            };
            handle_decode(params).await?;
        }

        WalletCommands::TxConfirmation { id } => {
            let params = ParamsWalletTxConfirmation {
                config,
                wallet_id,
                id: id.to_string(),
            };
            handle_tx_confirmation(params).await?;
        }

        WalletCommands::SimpleSend {
            address,
            value,
            change_address,
            token,
        } => {
            let params = ParamsWalletSimpleSend {
                config,
                wallet_id,
                address: address.to_string(),
                value: *value,
                change_address: change_address.clone(),
                token: token.clone(),
            };
            handle_simple_send(params).await?;
        }

        WalletCommands::Send { body } => {
            let params = ParamsWalletSend {
                config,
                wallet_id,
                body: body.to_string(),
            };
            handle_send(params).await?;
        }

        WalletCommands::CreateToken {
            name,
            symbol,
            amount,
            address,
            change_address,
            create_mint,
            mint_authority_address,
            allow_external_mint_authority_address,
            create_melt,
            melt_authority_address,
            allow_external_melt_authority_address,
            data,
        } => {
            let params = ParamsWalletCreateToken {
                config,
                wallet_id,
                name: name.to_string(),
                symbol: symbol.to_string(),
                amount: *amount,
                address: address.clone(),
                change_address: change_address.clone(),
                create_mint: *create_mint,
                mint_authority_address: mint_authority_address.clone(),
                allow_external_mint_authority_address: *allow_external_mint_authority_address,
                create_melt: *create_melt,
                melt_authority_address: melt_authority_address.clone(),
                allow_external_melt_authority_address: *allow_external_melt_authority_address,
                data: data.clone(),
            };
            handle_create_token(params).await?;
        }

        WalletCommands::MintTokens {
            token,
            amount,
            address,
            change_address,
            mint_authority_address,
            allow_external_mint_authority_address,
            unshift_data,
            data,
        } => {
            let params = ParamsWalletMintTokens {
                config,
                wallet_id,
                token: token.to_string(),
                amount: *amount,
                address: address.clone(),
                change_address: change_address.clone(),
                mint_authority_address: mint_authority_address.clone(),
                allow_external_mint_authority_address: *allow_external_mint_authority_address,
                unshift_data: *unshift_data,
                data: data.clone(),
            };
            handle_mint_tokens(params).await?;
        }

        WalletCommands::MeltTokens {
            token,
            amount,
            address,
            deposit_address,
            change_address,
            melt_authority_address,
            allow_external_melt_authority_address,
            unshift_data,
            data,
        } => {
            let params = ParamsWalletMeltTokens {
                config,
                wallet_id,
                token: token.to_string(),
                amount: *amount,
                address: address.clone(),
                deposit_address: deposit_address.clone(),
                change_address: change_address.clone(),
                melt_authority_address: melt_authority_address.clone(),
                allow_external_melt_authority_address: *allow_external_melt_authority_address,
                unshift_data: *unshift_data,
                data: data.clone(),
            };
            handle_melt_tokens(params).await?;
        }

        WalletCommands::UtxoFilter {
            max_utxos,
            token,
            filter_address,
            amount_smaller_than,
            amount_bigger_than,
            maximum_amount,
            only_available_utxos,
        } => {
            let params = ParamsWalletUtxoFilter {
                config,
                wallet_id,
                max_utxos: *max_utxos,
                token: token.clone(),
                filter_address: filter_address.clone(),
                amount_smaller_than: *amount_smaller_than,
                amount_bigger_than: *amount_bigger_than,
                maximum_amount: *maximum_amount,
                only_available_utxos: *only_available_utxos,
            };
            handle_utxo_filter(params).await?;
        }

        WalletCommands::UtxoConsolidation {
            max_utxos,
            token,
            filter_address,
            amount_smaller_than,
            amount_bigger_than,
            maximum_amount,
        } => {
            let params = ParamsWalletUtxoConsolidation {
                config,
                wallet_id,
                max_utxos: *max_utxos,
                token: token.clone(),
                filter_address: filter_address.clone(),
                amount_bigger_than: *amount_bigger_than,
                amount_smaller_than: *amount_smaller_than,
                maximum_amount: *maximum_amount,
            };
            handle_utxo_consolidation(params).await?;
        }

        WalletCommands::CreateNft {
            name,
            symbol,
            amount,
            data,
            address,
            change_address,
            create_mint,
            mint_authority_address,
            allow_external_mint_authority_address,
            create_melt,
            melt_authority_address,
            allow_external_melt_authority_address,
        } => {
            let params = ParamsWalletCreateNft {
                config,
                wallet_id,
                name: name.to_string(),
                symbol: symbol.to_string(),
                amount: *amount,
                data: data.to_string(),
                address: address.clone(),
                change_address: change_address.clone(),
                create_mint: *create_mint,
                mint_authority_address: mint_authority_address.clone(),
                allow_external_mint_authority_address: *allow_external_mint_authority_address,
                create_melt: *create_melt,
                melt_authority_address: melt_authority_address.clone(),
                allow_external_melt_authority_address: *allow_external_melt_authority_address,
            };
            handle_create_nft(params).await?;
        }

        WalletCommands::Stop {} => {
            let params = ParamsWalletStop { config, wallet_id };
            handle_stop(params).await?;
        }

        WalletCommands::P2sh { command } => {
            handle_p2sh_txproposal(config, wallet_id, command).await?;
        }
    }

    Ok(())
}

/////////////////////////////////////////// Main

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let config = CliConfig {
        host: cli.host,
        debug: cli.debug,
    };

    // Configure logging using the default RUST_LOG envvar
    if cli.debug && env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "trace");
    }
    env_logger::init();

    let result: Result<(), Box<dyn std::error::Error>> = match &cli.command {
        Some(Commands::Start {
            wallet_id,
            seed_key,
            passphrase,
            scan_policy,
            gap_limit,
            policy_start_index,
            policy_end_index,
            history_sync_mode,
            multisig,
            multisig_key,
        }) => {
            let params = ParamsStart {
                config,
                wallet_id: wallet_id.to_string(),
                seed_key: seed_key.to_string(),
                passphrase: passphrase.clone(),
                scan_policy: scan_policy.clone(),
                gap_limit: *gap_limit,
                policy_start_index: *policy_start_index,
                policy_end_index: *policy_end_index,
                history_sync_mode: history_sync_mode.clone(),
                multisig: *multisig,
                multisig_key: multisig_key.clone(),
            };
            handle_start(params).await
        }
        Some(Commands::MultisigPubkey {
            seed_key,
            passphrase,
        }) => {
            let params = ParamsMultisigPubkey {
                config,
                seed_key: seed_key.to_string(),
                passphrase: passphrase.clone(),
            };
            handle_multisig_pubkey(params).await
        }
        Some(Commands::ConfigurationString { token }) => {
            let params = ParamsConfigString {
                config,
                token: token.to_string(),
            };
            handle_configuration_string(params).await
        }
        Some(Commands::Wallet { wallet_id, command }) => {
            handle_wallet(config, wallet_id.to_string(), command).await
        }

        Some(Commands::Hsm { command }) => handle_hsm(config, command).await,

        Some(Commands::Fireblocks { command }) => handle_fireblocks(config, command).await,

        Some(Commands::Custom { command }) => handle_custom(config, command).await,

        None => {
            return Ok(());
        }
    };

    if let Err(err) = result {
        println!("{}", err);
    }

    Ok(())
}
