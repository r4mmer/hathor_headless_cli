pub mod params;
pub mod handler;

use params::*;
use handler::*;

use clap::{Parser, Subcommand};
use env_logger;
use log;

/////////////////////////////////////////// CLI structure

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, default_value = "http://localhost:8000")]
    host: String,

    #[arg(long, default_value_t = false)]
    debug: bool,

    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Start {
        #[arg(long, default_value = "default")]
        wallet_id: String,
        #[arg(long)]
        seed_key: String,
        #[arg(short, long)]
        passphrase: Option<String>,
    },

    MultisigPubkey  {
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
        command: WalletCommands
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
    }

    // mint-tokens
    // melt-tokens
    // utxo-filter
    // utxo-consolidation
    // create-nft
    // stop
}



async fn handle_wallet(config: CliConfig, wallet_id: &String, wallet_cmd: &WalletCommands) -> Result<(), Box<dyn std::error::Error>> {
    match &wallet_cmd {
        WalletCommands::Status {  } => {
            handle_status(config, wallet_id.to_string()).await?;
        }

        WalletCommands::Balance { token } => {
            let params = ParamsWalletBalance {
                config,
                wallet_id: wallet_id.to_string(),
                token: token.clone(),
            };
            handle_balance(params).await?;
        }

        WalletCommands::Address { index, mark_as_used } => {
            let params = ParamsWalletAddress {
                config,
                wallet_id: wallet_id.to_string(),
                index: index.clone(),
                mark_as_used: mark_as_used.clone(),
            };
            handle_address(params).await?;
        }

        WalletCommands::AddressInfo { address, token } => {
            let params = ParamsWalletAddressInfo {
                config,
                wallet_id: wallet_id.to_string(),
                address: address.to_string(),
                token: token.clone(),
            };
            handle_address_info(params).await?;
        }

        WalletCommands::AddressIndex { address } => {
            let params = ParamsWalletAddressIndex {
                config,
                wallet_id: wallet_id.to_string(),
                address: address.to_string(),
            };
            handle_address_index(params).await?;
        }

        WalletCommands::Addresses {  } => {
            let params = ParamsWalletAddresses {
                config,
                wallet_id: wallet_id.to_string(),
            };
            handle_addresses(params).await?;
        }

        WalletCommands::TxHistory { limit } => {
            let params = ParamsWalletTxHistory {
                config,
                wallet_id: wallet_id.to_string(),
                limit: limit.clone(),
            };
            handle_tx_history(params).await?;
        }

        WalletCommands::Transaction { id } => {
            let params = ParamsWalletTransaction {
                config,
                wallet_id: wallet_id.to_string(),
                id: id.to_string(),
            };
            handle_transaction(params).await?;
        }

        WalletCommands::Decode { tx_hex, partial_tx } => {
            let params = ParamsWalletDecode {
                config,
                wallet_id: wallet_id.to_string(),
                tx_hex: tx_hex.clone(),
                partial_tx: partial_tx.clone(),
            };
            handle_decode(params).await?;
        }

        WalletCommands::TxConfirmation { id } => {
            let params = ParamsWalletTxConfirmation {
                config,
                wallet_id: wallet_id.to_string(),
                id: id.to_string(),
            };
            handle_tx_confirmation(params).await?;
        }

        WalletCommands::SimpleSend { address, value, change_address, token } => {
            let params = ParamsWalletSimpleSend {
                config,
                wallet_id: wallet_id.to_string(),
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
                wallet_id: wallet_id.to_string(),
                body: body.to_string(),
            };
            handle_send(params).await?;
        }

        WalletCommands::CreateToken { name, symbol, amount, address, change_address, create_mint, mint_authority_address, allow_external_mint_authority_address, create_melt, melt_authority_address, allow_external_melt_authority_address } => {
            let params = ParamsWalletCreateToken {
                config,
                wallet_id: wallet_id.to_string(),
                name: name.to_string(),
                symbol: symbol.to_string(),
                amount: *amount,
                address: address.clone(),
                change_address: change_address.clone(),
                create_mint: create_mint.clone(),
                mint_authority_address: mint_authority_address.clone(),
                allow_external_mint_authority_address: allow_external_mint_authority_address.clone(),
                create_melt: create_melt.clone(),
                melt_authority_address: melt_authority_address.clone(),
                allow_external_melt_authority_address: allow_external_melt_authority_address.clone(),
            };
            handle_create_token(params).await?;
        }

    }

    Ok(())
}


/////////////////////////////////////////// Main

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let cli = Cli::parse();
    let result: Result<(), Box<dyn std::error::Error>>;

    let config = CliConfig {
        host: cli.host,
        debug: cli.debug,
    };

    // Configure logging using the default RUST_LOG envvar
    let mut log_builder = env_logger::Builder::from_default_env();
    if cli.debug {
        // Force trace logging to debug reqwest data
        log_builder.filter_level(log::LevelFilter::Trace);
    }
    log_builder.init();

    match &cli.command {
        Some(Commands::Start { wallet_id, seed_key, passphrase }) => {
            let params = ParamsStart {
                config,
                wallet_id: wallet_id.to_string(),
                seed_key: seed_key.to_string(),
                passphrase: passphrase.clone(),
            };
            result = handle_start(params).await;
        }
        Some(Commands::MultisigPubkey { seed_key, passphrase }) => {
            let params = ParamsMultisigPubkey {
                config,
                seed_key: seed_key.to_string(),
                passphrase: passphrase.clone(),
            };
            result = handle_multisig_pubkey(params).await;
        }
        Some(Commands::ConfigurationString { token }) => {
            let params = ParamsConfigString { config, token: token.to_string() };
            result = handle_configuration_string(params).await;
        }
        Some(Commands::Wallet { wallet_id, command }) => {
            result = handle_wallet(config, wallet_id, command).await;
        }
        None => {
            return Ok(());
        }
    }

    if let Err(err) = result {
        println!("{}", err);
    }

    Ok(())
}
