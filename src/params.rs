

/////////////////////////////////////////// handlers params

/// The common configuration of all commands in the cli
pub struct CliConfig {
    /// The base_url to send requests
    pub host: String,
    /// Enable reqwest trace logging
    pub debug: bool,
}

/// Arguments for the start command
pub struct ParamsStart {
    /// Common config
    pub config: CliConfig,
    /// wallet-id used to name the wallet for subsequent calls
    pub wallet_id: String,
    /// seed-key used to generate the wallet
    pub seed_key: String,
    /// Optional passphrase to use with the given seed
    pub passphrase: Option<String>,
}

/// Arguments for the multisig_pubkey command
pub struct ParamsMultisigPubkey {
    /// Common config
    pub config: CliConfig,
    /// Seed key to use when generating the xpubkey
    pub seed_key: String,
    /// Optional passphrase to include
    pub passphrase: Option<String>,
}

/// Arguments for the configuration-string command
pub struct ParamsConfigString {
    /// Common config
    pub config: CliConfig,
    /// Token to get the configuration string
    pub token: String,
}

/// Arguments for the wallet balance command
pub struct ParamsWalletBalance {
    /// Common config
    pub config: CliConfig,
    /// wallet-id used to identify the wallet
    pub wallet_id: String,
    /// Get the balance of this token, headless will default to HTR
    pub token: Option<String>,
}

/// Arguments for the wallet address command
pub struct ParamsWalletAddress {
    /// Common config
    pub config: CliConfig,
    /// wallet-id used to identify the wallet
    pub wallet_id: String,
    /// Optional index, to fetch a specific address
    pub index: Option<u32>,
    /// Optionally mark the address as used when retrieving it
    pub mark_as_used: Option<bool>,
}

/// Arguments for the wallet tx-history command
pub struct ParamsWalletTxHistory {
    /// Common config
    pub config: CliConfig,
    /// wallet-id used to identify the wallet
    pub wallet_id: String,
    /// Optionally limit the number of entries retrieved, will not impact performance
    pub limit: Option<u32>,
}

/// Arguments for the wallet address-info command
pub struct ParamsWalletAddressInfo {
    /// Common config
    pub config: CliConfig,
    /// wallet-id used to identify the wallet
    pub wallet_id: String,
    /// address to get the info for
    pub address: String,
    /// Optionally specify a custom token, headless defaults to HTR
    pub token: Option<String>,
}

/// Arguments for the wallet address-index command
pub struct ParamsWalletAddressIndex {
    /// Common config
    pub config: CliConfig,
    /// wallet-id used to indentify the wallet
    pub wallet_id: String,
    /// address to find the index of
    pub address: String,
}

/// Arguments for the wallet addresses command
pub struct ParamsWalletAddresses {
    /// Common config
    pub config: CliConfig,
    /// wallet-id used to indentify the wallet
    pub wallet_id: String,
}

/// Arguments for the wallet transaction command
pub struct ParamsWalletTransaction {
    /// Common config
    pub config: CliConfig,
    /// wallet-id used to indentify the wallet
    pub wallet_id: String,
    /// Transaction id to find the details for
    pub id: String,
}

/// Arguments for the wallet decode  command
pub struct ParamsWalletDecode {
    /// Common config
    pub config: CliConfig,
    /// wallet-id used to indentify the wallet
    pub wallet_id: String,
    /// tx_hex to decode
    pub tx_hex: Option<String>,
    /// partial_tx to decode
    pub partial_tx: Option<String>,
}

/// Arguments for the wallet tx-confirmation command
pub struct ParamsWalletTxConfirmation {
    /// Common config
    pub config: CliConfig,
    /// wallet-id used to indentify the wallet
    pub wallet_id: String,
    /// tx_id to get confirmation blocks from
    pub id: String,
}

/// Arguments for the wallet simple send command
pub struct ParamsWalletSimpleSend {
    /// Common config
    pub config: CliConfig,
    /// wallet-id used to indentify the wallet
    pub wallet_id: String,
    /// Address to send tokens
    pub address: String,
    /// Amount of tokens to send
    pub value: u32,
    /// Use this address if we require a change output
    pub change_address: Option<String>,
    /// Send this token, headless defaults to HTR
    pub token: Option<String>,
}

/// Arguments for the wallet send command
pub struct ParamsWalletSend {
    /// Common config
    pub config: CliConfig,
    /// wallet-id used to indentify the wallet
    pub wallet_id: String,
    /// json body to send, we perform no validations and send the body as is.
    /// The headless may reject the call if the body is not valid json or a valid transaction.
    pub body: String,
}

/// Arguments for the wallet create token command
pub struct ParamsWalletCreateToken {
    /// Common config
    pub config: CliConfig,
    /// wallet-id used to indentify the wallet
    pub wallet_id: String,
    /// Name of the token to create
    pub name: String,
    /// Symbol of the token to create
    pub symbol: String,
    /// amount of tokens to create
    pub amount: u32,
    /// Optionally specify the destination address
    pub address: Option<String>,
    /// Optionally specify the change address
    pub change_address: Option<String>,
    /// Create mint authority for the wallet, headless defaults to true
    pub create_mint: Option<bool>,
    /// Optionally send the mint authority to this address
    pub mint_authority_address: Option<String>,
    /// Flag to allow sending the mint authority to an address not from the wallet
    pub allow_external_mint_authority_address: Option<bool>,
    /// Create melt authority for the wallet, headless defaults to true
    pub create_melt: Option<bool>,
    /// Optionally send the melt authority to this address
    pub melt_authority_address: Option<String>,
    /// Flag to allow sending the melt authority to an address not from the wallet
    pub allow_external_melt_authority_address: Option<bool>,
}
