/////////////////////////////////////////// handlers params

/// The common configuration of all commands in the cli
#[derive(Clone, Debug)]
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
    /// Address scan policy
    pub scan_policy: Option<String>,
    /// Gap limit, only if the scan policy is "gap-limit"
    pub gap_limit: Option<u32>,
    /// Policy start index, only if the scan policy is "index-limit"
    pub policy_start_index: Option<u32>,
    /// Policy end index, only if the scan policy is "index-limit"
    pub policy_end_index: Option<u32>,
    /// History sync mode
    pub history_sync_mode: Option<String>,
    /// Multisig, if we want to start a multisig wallet
    pub multisig: bool,
    /// Multisig key, use this key on the multisig config instead of the seed-key
    pub multisig_key: Option<String>,
    // xpubkey, seed, precalculatedAddresses?
}

/// Arguments for the HSM start command
pub struct ParamsHsmStart {
    /// Common config
    pub config: CliConfig,
    /// wallet-id used to name the wallet for subsequent calls
    pub wallet_id: String,
    /// seed-key used to generate the wallet
    pub hsm_key: String,
}

/// Arguments for the Fireblocks start command
pub struct ParamsFireblocksStart {
    /// Common config
    pub config: CliConfig,
    /// wallet-id used to name the wallet for subsequent calls
    pub wallet_id: String,
    /// xpub used to generate the wallet
    pub xpub: String,
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
    pub data: Option<Vec<String>>,
}

/// Arguments for the wallet mint tokens command
pub struct ParamsWalletMintTokens {
    /// Common config
    pub config: CliConfig,
    /// wallet-id used to indentify the wallet
    pub wallet_id: String,
    /// Uid of the token
    pub token: String,
    /// amount of tokens to mint
    pub amount: u32,
    /// Optionally specify the destination address
    pub address: Option<String>,
    /// Optionally specify the change address
    pub change_address: Option<String>,
    /// Optionally send the mint authority to this address
    pub mint_authority_address: Option<String>,
    /// Flag to allow sending the mint authority to an address not from the wallet
    pub allow_external_mint_authority_address: Option<bool>,
    pub unshift_data: Option<bool>,
    pub data: Option<Vec<String>>,
}

/// Arguments for the wallet melt tokens command
pub struct ParamsWalletMeltTokens {
    /// Common config
    pub config: CliConfig,
    /// wallet-id used to indentify the wallet
    pub wallet_id: String,
    /// Uid of the token
    pub token: String,
    /// amount of tokens to melt
    pub amount: u32,
    /// Optionally specify the destination address
    pub address: Option<String>,
    /// Optionally specify the deposit address
    pub deposit_address: Option<String>,
    /// Optionally specify the change address
    pub change_address: Option<String>,
    /// Optionally send the melt authority to this address
    pub melt_authority_address: Option<String>,
    /// Flag to allow sending the melt authority to an address not from the wallet
    pub allow_external_melt_authority_address: Option<bool>,
    pub unshift_data: Option<bool>,
    pub data: Option<Vec<String>>,
}

/// Arguments for the wallet utxo filter command
pub struct ParamsWalletUtxoFilter {
    /// Common config
    pub config: CliConfig,
    /// wallet-id used to indentify the wallet
    pub wallet_id: String,
    /// Maximum number of utxos to return
    pub max_utxos: Option<u32>,
    /// Find utxos of this token
    pub token: Option<String>,
    /// Filter for utxos on this address
    pub filter_address: Option<String>,
    /// Only utxos with amount smaller than this
    pub amount_smaller_than: Option<u32>,
    /// Only utxos with amount smaller than this
    pub amount_bigger_than: Option<u32>,
    /// Maximum sum of tokens from returned utxos
    pub maximum_amount: Option<u32>,
    /// Only return unlocked and ready to be used utxos, headless defaults to true
    pub only_available_utxos: Option<bool>,
}

/// Arguments for the wallet utxo consolidation command
pub struct ParamsWalletUtxoConsolidation {
    /// Common config
    pub config: CliConfig,
    /// wallet-id used to indentify the wallet
    pub wallet_id: String,
    /// Maximum number of utxos to return
    pub max_utxos: Option<u32>,
    /// Find utxos of this token
    pub token: Option<String>,
    /// Filter for utxos on this address
    pub filter_address: Option<String>,
    /// Only utxos with amount smaller than this
    pub amount_smaller_than: Option<u32>,
    /// Only utxos with amount smaller than this
    pub amount_bigger_than: Option<u32>,
    /// Maximum sum of tokens from returned utxos
    pub maximum_amount: Option<u32>,
}

/// Arguments for the wallet create nft command
pub struct ParamsWalletCreateNft {
    /// Common config
    pub config: CliConfig,
    /// wallet-id used to indentify the wallet
    pub wallet_id: String,
    /// Name of the nft to create
    pub name: String,
    /// Symbol of the nft to create
    pub symbol: String,
    /// amount of nfts to create
    pub amount: u32,
    /// Data for the NFT's data output
    pub data: String,
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

pub struct ParamsWalletStop {
    pub config: CliConfig,
    pub wallet_id: String,
}

pub struct ParamsCustomListTokens {
    pub config: CliConfig,
    pub wallet_id: String,
}

pub struct ParamsCustomCurl {
    pub config: CliConfig,
    pub wallet_id: String,
    pub post: bool,
    pub data: bool,
    pub path: String,
}

pub struct ParamsP2shTxProposalBuild {
    pub config: CliConfig,
    pub wallet_id: String,
    pub body: String,
}

pub struct ParamsP2shTxProposalBuildSimpleSendTokens {
    pub config: CliConfig,
    pub wallet_id: String,
    pub address: String,
    pub value: u32,
    pub token: Option<String>,
    pub change_address: Option<String>,
    pub mark_inputs_as_used: Option<bool>,
}

pub struct ParamsP2shTxProposalGetMySignatures {
    pub config: CliConfig,
    pub wallet_id: String,
    pub tx_hex: String,
}
pub struct ParamsP2shTxProposalSign {
    pub config: CliConfig,
    pub wallet_id: String,
    pub tx_hex: String,
    pub signatures: Vec<String>,
}

/// Arguments for the wallet create token command
pub struct ParamsWalletP2shTxProposalCreateToken {
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
    /// Flag to mark inputs as used
    pub mark_inputs_as_used: Option<bool>,
}

/// Arguments for the wallet P2SH mint tokens command
pub struct ParamsWalletP2shTxProposalMintTokens {
    /// Common config
    pub config: CliConfig,
    /// wallet-id used to indentify the wallet
    pub wallet_id: String,
    /// Uid of the token
    pub token: String,
    /// amount of tokens to mint
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
    /// Flag to mark inputs as used
    pub mark_inputs_as_used: Option<bool>,
}
