use crate::types::{
    Collateral, CreateVaultInput, CreateVaultReceipt, Vault, VaultCollection, VaultErr, VaultId,
    VaultState,
};
use bitcoin::{secp256k1, Address, Network, PrivateKey};
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
    /// todo: how to prevent excessive vault creation attacks, without anyone actually claiming tokens later
    /// alternatively, we don't use direct bitcoin. but instead, use "ckBTC", some ICP wrapped bitcoin.
    pub fn create_vault(
        &mut self,
        principal: Principal,
        _input: CreateVaultInput,
    ) -> CreateVaultReceipt {
        let id = self.next_id();

        ic_cdk::println!("spare keys {:?}", self.spare_keys);

        let pk = self.spare_keys.pop().ok_or(VaultErr::MissingPrivateKey)?;

        ic_cdk::println!("using key {:?}", pk);

        match self.vaults.insert(
            id,
            Vault {
                id,
                collateral: Collateral::BTC,
                debt: 100,
                liquidation_price: 500,
                maintenance_ratio: 100,
                interest_rate: 0,
                owner: principal,
                state: VaultState::Open,
                private_key: pk.to_string(),
            },
        ) {
            Some(_) => Err(VaultErr::Conflict),
            None => self
                .vaults
                .get(&id)
                .ok_or(VaultErr::NotFound)
                .and_then(|v| Ok(v.clone())),
        }
    }

    pub fn get_vault(&self, id: VaultId) -> Option<Vault> {
        self.vaults.get(&id).cloned()
    }

    fn next_id(&mut self) -> VaultId {
        self.next_id += 1;
        self.next_id
    }
}

impl Vault {
    /// Returns the regtest P2PKH address derived from the private key.
    pub fn btc_address(&self, network: Network) -> Address {
        let private_key = PrivateKey::from_wif(&self.private_key).unwrap();
        let public_key = private_key.public_key(&secp256k1::Secp256k1::new());
        Address::p2pkh(&public_key, network)
    }
}

impl VaultCollection {
    fn new() -> VaultCollection {
        VaultCollection(Vec::new())
    }

    fn add(&mut self, elem: Vault) {
        self.0.push(elem);
    }
}

impl FromIterator<Vault> for VaultCollection {
    fn from_iter<I: IntoIterator<Item = Vault>>(iter: I) -> Self {
        let mut c = VaultCollection::new();

        for i in iter {
            c.add(i);
        }

        c
    }
}

pub const BTC_SPARE_PRIVATE_KEYS: [&str; 5] = [
    "L2C1QgyKqNgfV7BpEPAm6PVn2xW8zpXq6MojSbWdH18nGQF2wGsT",
    "Ky3BLwXx7ouVJSQ7P28KFTsxfH6RN86xrdqYdzSe7m2p3gp83dza",
    "L19t4zqFrzfmtgzFd1uZmeKY8UrXzXuHzmZUjswZKYUuUtkmiaBE",
    "KxarCFNSxu1kbMfxqJ1MPxtghsamnos62vV1XG9HqvpHSxdYkXU5",
    "KwyPiCJvGTHfVnnwittkNWxQVQr1zK9gVN2cjJfW4W9sER97W3Dc",
];
