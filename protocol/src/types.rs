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
    pub interest_rate: u64,
    pub state: VaultState,
    /// @todo remove once vaults can generate ecdsa
    pub private_key: String,
}

#[derive(CandidType, Clone, Debug)]
pub struct VaultBTC {
    pub public_address: String,
    pub balance: u64,
}

pub struct VaultCollection(pub Vec<Vault>);

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

pub type CreateVaultReceipt = Result<Vault, VaultErr>;
pub type ClaimVaultReceipt = Result<u64, VaultErr>;

#[derive(CandidType, Debug)]
pub enum VaultErr {
    MissingPrivateKey,
    NotFound,
    Conflict,
    Bad(String),
    Unknown,
    InsufficientAmount,
    InvalidBalance,
}

#[derive(CandidType, Debug)]
pub enum OracleErr {
    Fail,
}
