use crate::types::{Collateral, CreateVaultInput};
use crate::types::{Vault, VaultId, VaultState};
use ic_cdk::export::Principal;
use std::collections::HashMap;

type Vaults = HashMap<VaultId, Vault>;

#[derive(Default, Clone)]
pub struct VaultManager {
    pub next_id: VaultId,
    pub vaults: Vaults,
}

impl VaultManager {
    pub fn create_vault(&mut self, principal: Principal, _input: CreateVaultInput) -> VaultId {
        let id = self.next_id();

        self.vaults.insert(
            id,
            Vault {
                id,
                collateral: Collateral::BTC,
                debt: 100,
                liquidation_price: 500,
                maintenance_ratio: 100,
                owner: principal,
                state: VaultState::Open,
            },
        );
        id
    }

    pub fn get_vault(&self, id: VaultId) -> Option<Vault> {
        self.vaults.get(&id).cloned()
    }

    fn next_id(&mut self) -> VaultId {
        self.next_id += 1;
        self.next_id
    }
}
