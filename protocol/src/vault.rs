use crate::types::{
    Collateral, CreateVaultErr, CreateVaultInput, CreateVaultReceipt, Vault, VaultId, VaultState,
};
use ic_cdk::export::Principal;
use std::collections::HashMap;

type Vaults = HashMap<VaultId, Vault>;

#[derive(Default, Clone)]
pub struct VaultManager {
    pub next_id: VaultId,
    pub vaults: Vaults,
    pub spare_keys: Vec<&'static str>,
}

impl VaultManager {
    pub fn create_vault(
        &mut self,
        principal: Principal,
        _input: CreateVaultInput,
    ) -> CreateVaultReceipt {
        let id = self.next_id();

        let pk = self.spare_keys.pop().ok_or(CreateVaultErr::MissingKeys)?;

        let vault = self
            .vaults
            .insert(
                id,
                Vault {
                    id,
                    collateral: Collateral::BTC,
                    debt: 100,
                    liquidation_price: 500,
                    maintenance_ratio: 100,
                    owner: principal,
                    state: VaultState::Open,
                    private_key: pk.to_string(),
                    btc_public_address: "".to_string(),
                },
            )
            .ok_or(CreateVaultErr::BadVault("Unable to insert vault".to_string()));

        vault
    }

    pub fn get_vault(&self, id: VaultId) -> Option<Vault> {
        self.vaults.get(&id).cloned()
    }

    fn next_id(&mut self) -> VaultId {
        self.next_id += 1;
        self.next_id
    }
}

pub const BTC_SPARE_PRIVATE_KEYS: [&'static str; 5] = [
    "L2C1QgyKqNgfV7BpEPAm6PVn2xW8zpXq6MojSbWdH18nGQF2wGsT",
    "Ky3BLwXx7ouVJSQ7P28KFTsxfH6RN86xrdqYdzSe7m2p3gp83dza",
    "L19t4zqFrzfmtgzFd1uZmeKY8UrXzXuHzmZUjswZKYUuUtkmiaBE",
    "KxarCFNSxu1kbMfxqJ1MPxtghsamnos62vV1XG9HqvpHSxdYkXU5",
    "KwyPiCJvGTHfVnnwittkNWxQVQr1zK9gVN2cjJfW4W9sER97W3Dc",
];
