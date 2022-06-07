use crate::types::CreateVaultErr;
use ic_cdk::{
    api::call::call_with_payment,
    export::{candid::CandidType, Principal},
};
use serde::Deserialize;

#[derive(CandidType, Deserialize, Debug)]
struct CreateCanisterPayload {
    canister_id: Principal,
}

pub async fn create_canister() -> Result<Principal, CreateVaultErr> {
    let r = call_with_payment::<(), (CreateCanisterPayload, ())>(
        Principal::management_canister(),
        "create_canister",
        (),
        1000000,
    )
    .await;
    let payload = r.map_err(|err| CreateVaultErr::Bad(err.1))?;

    ic_cdk::println!("creating new canister {:#?}", payload.0.canister_id);

    Ok(payload.0.canister_id)
}
