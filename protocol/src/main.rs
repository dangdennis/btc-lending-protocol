use ic_cdk::caller;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_cdk_macros::{init, query, update};
use std::cell::RefCell;
use types::{Vault, VaultId};

mod types;
mod vault;

use vault::{CreateVaultInput, VaultManager};

thread_local! {
    static BTC_CANISTER_ID: RefCell<Principal> = RefCell::new(Principal::management_canister());
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(Default)]
pub struct State {
    owner: Option<Principal>,
    vault_manager: VaultManager,
}

#[derive(CandidType, Deserialize)]
struct InitPayload {
    bitcoin_canister_id: Principal,
}

#[init]
fn init(payload: InitPayload) {
    ic_cdk::setup();
    BTC_CANISTER_ID.with(|id| {
        id.replace(payload.bitcoin_canister_id);
    });
    STATE.with(|s| {
        s.borrow_mut().owner = Some(caller());
    });
}

#[update]
fn create_vault(input: CreateVaultInput) -> VaultId {
    let caller = caller();
    STATE.with(|s| s.borrow_mut().vault_manager.create_vault(caller, input))
}

#[query]
fn get_vault(id: VaultId) -> Option<Vault> {
    STATE.with(|s| s.borrow().vault_manager.get_vault(id))
}

fn main() {}

#[derive(Debug)]
pub struct BitcoinKeyPairs {
    private_key: &'static str,
    used: bool,
}

const BTC_SPARE_PRIVATE_KEYS: [BitcoinKeyPairs; 5] = [
    BitcoinKeyPairs {
        private_key: "L2C1QgyKqNgfV7BpEPAm6PVn2xW8zpXq6MojSbWdH18nGQF2wGsT",
        used: false,
    },
    BitcoinKeyPairs {
        private_key: "Ky3BLwXx7ouVJSQ7P28KFTsxfH6RN86xrdqYdzSe7m2p3gp83dza",
        used: false,
    },
    BitcoinKeyPairs {
        private_key: "L19t4zqFrzfmtgzFd1uZmeKY8UrXzXuHzmZUjswZKYUuUtkmiaBE",
        used: false,
    },
    BitcoinKeyPairs {
        private_key: "KxarCFNSxu1kbMfxqJ1MPxtghsamnos62vV1XG9HqvpHSxdYkXU5",
        used: false,
    },
    BitcoinKeyPairs {
        private_key: "KwyPiCJvGTHfVnnwittkNWxQVQr1zK9gVN2cjJfW4W9sER97W3Dc",
        used: false,
    },
];
