use ic_cdk::export::{candid::Nat, Principal};
use std::collections::HashMap;

type Wallets = HashMap<Principal, Nat>;

#[derive(Default, Clone)]
pub struct WalletManager {
    pub wallets: Wallets,
}
