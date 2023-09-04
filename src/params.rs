

/////////////////////////////////////////// handlers params

pub struct CliConfig {
    pub host: String,
}

pub struct ParamsStart {
    pub config: CliConfig,
    pub wallet_id: String,
    pub seed_key: String,
    pub passphrase: Option<String>,
}

pub struct ParamsMultisigPubkey {
    pub config: CliConfig,
    pub seed_key: String,
    pub passphrase: Option<String>,
}

pub struct ParamsConfigString {
    pub config: CliConfig,
    pub token: String,
}

pub struct ParamsWalletBalance {
    pub config: CliConfig,
    pub wallet_id: String,
    pub token: Option<String>,
}

pub struct ParamsWalletAddress {
    pub config: CliConfig,
    pub wallet_id: String,
    pub index: Option<u32>,
    pub mark_as_used: Option<bool>,
}

pub struct ParamsWalletTxHistory {
    pub config: CliConfig,
    pub wallet_id: String,
    pub limit: Option<u32>,
}

pub struct ParamsWalletAddressInfo {
    pub config: CliConfig,
    pub wallet_id: String,
    pub address: String,
    pub token: Option<String>,
}

pub struct ParamsWalletAddressIndex {
    pub config: CliConfig,
    pub wallet_id: String,
    pub address: String,
}

pub struct ParamsWalletAddresses {
    pub config: CliConfig,
    pub wallet_id: String,
}

pub struct ParamsWalletTransaction {
    pub config: CliConfig,
    pub wallet_id: String,
    pub id: String,
}

pub struct ParamsWalletDecode {
    pub config: CliConfig,
    pub wallet_id: String,
    pub tx_hex: Option<String>,
    pub partial_tx: Option<String>,
}

pub struct ParamsWalletTxConfirmation {
    pub config: CliConfig,
    pub wallet_id: String,
    pub id: String,
}

pub struct ParamsWalletSimpleSend {
    pub config: CliConfig,
    pub wallet_id: String,
    pub address: String,
    pub value: u32,
    pub change_address: Option<String>,
    pub token: Option<String>,
}

pub struct ParamsWalletSend {
    pub config: CliConfig,
    pub wallet_id: String,
    pub body: String,
}

pub struct ParamsWalletCreateToken {
    pub config: CliConfig,
    pub wallet_id: String,
    pub name: String,
    pub symbol: String,
    pub amount: u32,
    pub address: Option<String>,
    pub change_address: Option<String>,
    pub create_mint: Option<bool>,
    pub mint_authority_address: Option<String>,
    pub allow_external_mint_authority_address: Option<bool>,
    pub create_melt: Option<bool>,
    pub melt_authority_address: Option<String>,
    pub allow_external_melt_authority_address: Option<bool>,
}
