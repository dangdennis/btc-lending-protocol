use ic_cdk::export::{candid::CandidType, Principal};
use serde::Deserialize;

pub type VaultId = u32;

#[derive(CandidType, Clone)]
pub struct Vault {
    pub id: VaultId,
    pub collateral: Collateral,
    pub owner: Principal,
    pub maintenance_ratio: u64,
    pub debt: u64,
    pub liquidation_price: u64,
    pub state: VaultState,
    pub btc_public_address: String,
    pub private_key: String,
}

#[derive(CandidType, Clone)]
pub enum Collateral {
    BTC,
    ICP,
}

#[derive(CandidType, Clone)]
pub enum VaultState {
    Redeemed,
    Liquidated,
    Open,
}

#[derive(CandidType, Deserialize)]
pub struct CreateVaultInput {}

pub type CreateVaultReceipt = Result<Vault, CreateVaultErr>;

#[derive(CandidType)]
pub enum CreateVaultErr {
    MissingKeys,
    BadVault(String),
}
