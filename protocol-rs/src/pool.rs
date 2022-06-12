use ic_cdk::export::{candid::CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type PoolSum = u64;

#[derive(Default, Clone)]
pub struct StabilityPool(PoolSum, HashMap<Principal, u64>);
#[derive(Default, Clone)]
pub struct StakingPool(PoolSum, HashMap<Principal, u64>);

#[derive(CandidType, Serialize, Deserialize)]
pub enum PoolType {
    Stability,
    Staking,
}

#[derive(Default, Clone)]
pub struct PoolManager {
    pub stability_pool: StabilityPool,
    pub staking_pool: StakingPool,
}

impl PoolManager {
    pub fn get_tvl(&self, pool_type: PoolType) -> u64 {
        match pool_type {
            PoolType::Stability => self.stability_pool.0,
            PoolType::Staking => self.staking_pool.0,
        }
    }

    pub fn get_apr(&self, pool_type: PoolType) -> u64 {
        match pool_type {
            _ => 0,
        }
    }
}
