use bitcoin::Address;
use ic_btc_types::GetBalanceError;
use ic_btc_types::GetBalanceRequest;
use ic_cdk::api::call::RejectionCode;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_cdk::{call, caller};
use ic_cdk_macros::{init, query, update};
use pool::{PoolManager, PoolType};
use std::cell::RefCell;
use std::collections::HashMap;
use types::VaultBTC;
use types::{ClaimVaultReceipt, CreateVaultInput, CreateVaultReceipt, Vault, VaultErr, VaultId};
use vault::{VaultManager, BTC_SPARE_PRIVATE_KEYS};
use wallet::WalletManager;

mod management_canister;
mod oracle;
mod pool;
mod types;
mod vault;
mod wallet;

thread_local! {
    static BTC_CANISTER_ID: RefCell<Principal> = RefCell::new(Principal::management_canister());
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(Default)]
pub struct State {
    owner: Option<Principal>,
    vault_manager: VaultManager,
    pool_manager: PoolManager,
    wallet_manager: WalletManager,
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

    let new_vault = STATE.with(|s| {
        s.borrow_mut()
            .vault_manager
            .create_vault(caller, CreateVaultInput { ..input })
    });

    new_vault
}

#[query]
fn get_vaults() -> Vec<Vault> {
    STATE.with(|s| {
        s.borrow()
            .vault_manager
            .clone()
            .vaults
            .values()
            .cloned()
            .collect()
    })
}

#[query]
fn get_vault(id: VaultId) -> Option<Vault> {
    STATE.with(|s| s.borrow().vault_manager.get_vault(id))
}

#[query]
fn get_tvl(pool_type: PoolType) -> u64 {
    STATE.with(|s| s.borrow().pool_manager.get_tvl(pool_type))
}

#[query]
fn get_apr(pool_type: PoolType) -> u64 {
    STATE.with(|s| s.borrow().pool_manager.get_apr(pool_type))
}

#[query]
fn get_stake(_pool_type: PoolType) -> u64 {
    unimplemented!()
}

#[update]
async fn claim_vault(id: VaultId) -> ClaimVaultReceipt {
    let (wm, vm) = STATE.with(|s| {
        let wallet_manager = &s.borrow().wallet_manager;
        let vault_manager = &s.borrow().vault_manager;
        (wallet_manager.clone(), vault_manager.clone())
    });

    let btc_canister_id = get_btc_canister_id();

    let vault = vm.get_vault(id).ok_or(VaultErr::NotFound)?;
    let btc_usd_price = oracle::get_btc_price().map_err(|err| {
        ic_cdk::println!("oracle error {:?}", err);
        VaultErr::Bad("Failed".to_string())
    })?;

    let deposited_collateral: Result<(Result<u64, GetBalanceError>,), (RejectionCode, String)> =
        call(
            btc_canister_id,
            "get_balance",
            (GetBalanceRequest {
                address: vault.btc_address(bitcoin::Network::Regtest).to_string(),
                min_confirmations: Some(0),
            },),
        )
        .await;

    let am = deposited_collateral
        .map_err(|_| VaultErr::InvalidBalance)?
        .0
        .map_err(|_| VaultErr::InvalidBalance)?;

    ic_cdk::println!("deposited satoshi {:?}", am);

    // convert the oracle btc price to satoshi equivalent

    let expected_min_collateral =
        btc_usd_price * (1 / 100_000_000_000) * (100 * vault.interest_rate * vault.debt);
    let current_collateral = 0;

    // get bitcoin price from oracle
    // check if vault's collateral (based on bitcoin price) is greater than or equal to the desired balance
    // take borrow fee
    // wallet_manager.lend_token(vault); update user's stablecoin balance

    Ok(0)
}

#[query]
fn get_vault_btc_address(id: VaultId) -> Result<String, VaultErr> {
    let vault = STATE
        .with(|s| s.borrow().vault_manager.get_vault(id))
        .ok_or(VaultErr::NotFound)?;
    let btc_addr = vault.btc_address(bitcoin::Network::Regtest).to_string();
    Ok(btc_addr)
}

#[update]
async fn get_vault_btc(id: VaultId) -> Result<VaultBTC, VaultErr> {
    let vault = STATE
        .with(|s| s.borrow().vault_manager.get_vault(id))
        .ok_or(VaultErr::NotFound)?;

    let deposited_collateral: Result<(Result<u64, GetBalanceError>,), (RejectionCode, String)> =
        call(
            get_btc_canister_id(),
            "get_balance",
            (GetBalanceRequest {
                address: vault.btc_address(bitcoin::Network::Regtest).to_string(),
                min_confirmations: Some(0),
            },),
        )
        .await;

    let collateral = deposited_collateral
        .map_err(|_| VaultErr::InvalidBalance)?
        .0
        .map_err(|_| VaultErr::InvalidBalance)?;

    let btc_addr = vault.btc_address(bitcoin::Network::Regtest).to_string();

    Ok(VaultBTC {
        balance: collateral,
        public_address: btc_addr,
    })
}

fn main() {}

fn get_btc_canister_id() -> Principal {
    BTC_CANISTER_ID.with(|id| *id.borrow())
}
