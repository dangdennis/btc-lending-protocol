use crate::types::{
    Collateral, CreateVaultErr, CreateVaultInput, CreateVaultReceipt, Vault, VaultId, VaultState,
};
use bitcoin::{secp256k1, Address, Network, PrivateKey};
use ic_cdk::export::Principal;
use std::collections::HashMap;

type Vaults = HashMap<VaultId, Vault>;

impl Vault {
    /// Returns the regtest P2PKH address derived from the private key.
    pub fn btc_address(&self, network: Network) -> Address {
        let private_key = PrivateKey::from_wif(&self.private_key).unwrap();
        let public_key = private_key.public_key(&secp256k1::Secp256k1::new());
        Address::p2pkh(&public_key, network)
    }
}

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

        ic_cdk::println!("spare keys {:?}", self.spare_keys);

        let pk = self.spare_keys.pop().ok_or(CreateVaultErr::MissingKeys)?;

        ic_cdk::println!("using key {:?}", pk);

        let vault = Ok(Vault {
            id,
            collateral: Collateral::BTC,
            debt: 100,
            liquidation_price: 500,
            maintenance_ratio: 100,
            owner: principal,
            state: VaultState::Open,
            private_key: pk.to_string(),
        });

        // let vault = self
        //     .vaults
        //     .insert(
        //         id,
        //         Vault {
        //             id,
        //             collateral: Collateral::BTC,
        //             debt: 100,
        //             liquidation_price: 500,
        //             maintenance_ratio: 100,
        //             owner: principal,
        //             state: VaultState::Open,
        //             private_key: pk.to_string(),
        //         },
        //     )
        //     .ok_or(CreateVaultErr::Bad("Unable to insert vault".to_string()));

        ic_cdk::println!("{:?}", vault);

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
