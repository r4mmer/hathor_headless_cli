use serde::{Serialize, Deserialize};

/////////////////////////////////////////// Data structures

#[derive(Serialize, Deserialize, Debug)]
pub struct DecodedOutput {
    // pub type: String,
    pub address: Option<String>,
    pub timelock: Option<u64>,
    pub data: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryOutput {
    pub value: u32,
    pub token_data: u32,
    pub script: String,
    pub decoded: DecodedOutput,
    pub token: String,
    pub spent_by: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryInput {
    pub value: u32,
    pub token_data: u32,
    pub script: String,
    pub decoded: DecodedOutput,
    pub token: String,
    pub tx_id: String,
    pub index: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryTx {
    pub tx_id: String,
    pub version: u32,
    pub weight: f32,
    pub timestamp: u64,
    pub is_voided: bool,
    pub inputs: Vec<HistoryInput>,
    pub outputs: Vec<HistoryOutput>,
    pub parents: Vec<String>,
    pub token_name: Option<String>,
    pub token_symbol: Option<String>,
    pub tokens: Option<Vec<String>>,
}