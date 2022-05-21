use ic_cdk::export::{
    candid::{CandidType},
    Principal,
};

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
