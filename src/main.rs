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
    #[arg(long, default_value = "http://localhost:8000")]
    host: String,

    #[arg(long, default_value_t = false)]
    debug: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Start {
        #[arg(long, default_value = "default")]
        wallet_id: String,
        #[arg(long, default_value = "default")]
        seed_key: String,
        #[arg(short, long)]
        passphrase: Option<String>,
        #[arg(long)]
        scan_policy: Option<String>,
        #[arg(long)]
        gap_limit: Option<u32>,
        #[arg(long)]
        policy_start_index: Option<u32>,
        #[arg(long)]
        policy_end_index: Option<u32>,
        #[arg(long)]
        history_sync_mode: Option<String>,
    },

    MultisigPubkey {
        seed_key: String,
        #[arg(short, long)]
        passphrase: Option<String>,
    },

    ConfigurationString {
        token: String,
    },

    Wallet {
        #[arg(short, long, default_value = "default")]
        wallet_id: String,

        #[command(subcommand)]
        command: WalletCommands,
    },

    Hsm {
        #[command(subcommand)]
        command: HsmCommands,
    },

    Fireblocks {
        #[command(subcommand)]
        command: FireblocksCommands,
    },

    Custom {
        #[command(subcommand)]
        command: CustomCommands,
    },
}

#[derive(Subcommand)]
enum CustomCommands {
    ListTokens {
        #[arg(short, long, default_value = "default")]
        wallet_id: String,
    },

    Curl {
        #[arg(short, long, default_value = "default")]
        wallet_id: String,
        #[arg(short, long, default_value_t = false)]
        post: bool,
        #[arg(short, long, default_value_t = false)]
        data: bool,
        path: String,
    },
}

#[derive(Subcommand)]
enum HsmCommands {
    Start {
        hsm_key: String,

        #[arg(long, default_value = "default")]
        wallet_id: String,
        // Other start arguments are still not supported in the headless
    },
}

#[derive(Subcommand)]
enum FireblocksCommands {
    Start {
        xpub: String,

        #[arg(long, default_value = "default")]
        wallet_id: String,
        // Other start arguments are still not supported in the headless
    },
}

#[derive(Subcommand)]
enum WalletCommands {
    Balance {
        #[arg(short, long)]
        token: Option<String>,
    },

    Status {},

    Address {
        #[arg(short, long)]
        index: Option<u32>,

        #[arg(short, long)]
        mark_as_used: Option<bool>,
    },

    AddressIndex {
        address: String,
    },

    Addresses {},

    AddressInfo {
        address: String,

        #[arg(short, long)]
        token: Option<String>,
    },

    TxHistory {
        #[arg(short, long)]
        limit: Option<u32>,
    },

    Transaction {
        id: String,
    },

    Decode {
        #[arg(short, long)]
        tx_hex: Option<String>,

        #[arg(short, long)]
        partial_tx: Option<String>,
    },

    TxConfirmation {
        id: String,
    },

    SimpleSend {
        address: String,
        value: u32,
        #[arg(short, long)]
        change_address: Option<String>,
        #[arg(short, long)]
        token: Option<String>,
    },

    Send {
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
        data: Option<Vec<String>>,
    },

    MintTokens {
        token: String,
        amount: u32,
        #[arg(long)]
        address: Option<String>,
        #[arg(long)]
        change_address: Option<String>,
        #[arg(long)]
        mint_authority_address: Option<String>,
        #[arg(long)]
        allow_external_mint_authority_address: Option<bool>,
        #[arg(short, long)]
        unshift_data: Option<bool>,
        #[arg(short, long)]
        data: Option<Vec<String>>,
    },

    MeltTokens {
        token: String,
        amount: u32,
        #[arg(long)]
        address: Option<String>,
        #[arg(long)]
        deposit_address: Option<String>,
        #[arg(long)]
        change_address: Option<String>,
        #[arg(long)]
        melt_authority_address: Option<String>,
        #[arg(long)]
        allow_external_melt_authority_address: Option<bool>,
        #[arg(short, long)]
        unshift_data: Option<bool>,
        #[arg(short, long)]
        data: Option<Vec<String>>,
    },

    UtxoFilter {
        #[arg(long)]
        max_utxos: Option<u32>,
        #[arg(long)]
        token: Option<String>,
        #[arg(long)]
        filter_address: Option<String>,
        #[arg(long)]
        amount_smaller_than: Option<u32>,
        #[arg(long)]
        amount_bigger_than: Option<u32>,
        #[arg(long)]
        maximum_amount: Option<u32>,
        #[arg(long)]
        only_available_utxos: Option<bool>,
    },

    UtxoConsolidation {
        #[arg(long)]
        max_utxos: Option<u32>,
        #[arg(long)]
        token: Option<String>,
        #[arg(long)]
        filter_address: Option<String>,
        #[arg(long)]
        amount_smaller_than: Option<u32>,
        #[arg(long)]
        amount_bigger_than: Option<u32>,
        #[arg(long)]
        maximum_amount: Option<u32>,
    },

    CreateNft {
        name: String,
        symbol: String,
        amount: u32,
        data: String,
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
    },

    Stop {},
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
