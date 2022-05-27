use ic_cdk::caller;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_cdk_macros::{init, query, update};
use std::cell::RefCell;
use std::collections::HashMap;
use types::{CreateVaultInput, CreateVaultReceipt, Vault, VaultId};
use vault::{VaultManager, BTC_SPARE_PRIVATE_KEYS};

mod management_canister;
mod types;
mod vault;

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
    STATE.with(|s| {
        s.borrow_mut().vault_manager = VaultManager {
            spare_keys: BTC_SPARE_PRIVATE_KEYS.to_vec(),
            next_id: 0,
            vaults: HashMap::new(),
        }
    });
}

#[update]
async fn create_vault(input: CreateVaultInput) -> CreateVaultReceipt {
    let caller = caller();

    let id = STATE.with(|s| {
        s.borrow_mut()
            .vault_manager
            .create_vault(caller, CreateVaultInput { ..input })
    });
    let vault = STATE
        .with(|s| s.borrow().vault_manager.get_vault(id))
        .unwrap();

    Ok(vault)
}

#[query]
fn get_vault(id: VaultId) -> Option<Vault> {
    STATE.with(|s| s.borrow().vault_manager.get_vault(id))
}

fn main() {}
