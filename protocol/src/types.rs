use ic_cdk::export::{candid::CandidType, Principal};
use serde::Deserialize;

pub type VaultId = u32;

#[derive(CandidType, Clone, Debug)]
pub struct Vault {
    pub id: VaultId,
    pub collateral: Collateral,
    pub owner: Principal,
    pub maintenance_ratio: u64,
    pub debt: u64,
    pub liquidation_price: u64,
    pub state: VaultState,
    /// @todo remove once vaults can generate ecdsa
    pub private_key: String,
}

#[derive(CandidType, Clone, Debug)]
pub enum Collateral {
    BTC,
    ICP,
}

#[derive(CandidType, Clone, Debug)]
pub enum VaultState {
    Open,
    Borrowed,
    Redeemed,
    Liquidated,
}

#[derive(CandidType, Deserialize)]
pub struct CreateVaultInput {}

pub type CreateVaultReceipt = Result<Vault, CreateVaultErr>;

#[derive(CandidType, Debug)]
pub enum CreateVaultErr {
    MissingKeys,
    Bad(String),
    Unknown
}
