use ic_cdk::export::candid::{self, CandidType, Deserialize};
use ic_cdk::api::call::CallResult;
use candid::{Nat, Principal};

#[derive(CandidType, Deserialize)]
pub enum Error {
    CreateCanisterFailed(Nat),
    LedgerTransferFailed(Nat),
    Unauthorized,
}

#[derive(CandidType, Deserialize)]
pub enum CreateICSPResult {
    Ok(Principal),
    Err(Error)
}

#[derive(CandidType, Deserialize)]
pub struct TopUpArgs {
    icsp_canister_id: Principal,
    icp_amount: u64
}

#[derive(CandidType, Deserialize)]
pub enum TopUpResult {
    Ok,
    Err(Error)
}

#[derive(CandidType, Deserialize)]
pub struct TransformArgs {
    to_canister_id: Principal,
    icp_amount: u64
}

pub type ISP = candid::Service;

pub struct SERVICE(pub candid::Principal);
impl SERVICE{

    pub async fn add_admin(&self, new_admin: Principal) -> CallResult<(bool,)> {
        ic_cdk::call(self.0, "addAdmin", (new_admin,)).await
    }

    pub async fn change_admins(&self, new_admins: Vec<Principal>) -> CallResult<
        (bool,)
    > { ic_cdk::call(self.0, "changeAdmins", (new_admins,)).await }

    pub async fn clear_log(&self) -> CallResult<()> {
        ic_cdk::call(self.0, "clearLog", ()).await
    }

    pub async fn create_icsp(&self, name: String, amount: u64) -> CallResult<
        (CreateICSPResult,)
    > { ic_cdk::call(self.0, "createICSP", (name,amount,)).await }

    pub async fn get_admins(&self) -> CallResult<(Vec<Principal>,)> {
        ic_cdk::call(self.0, "getAdmins", ()).await
    }

    pub async fn get_log(&self) -> CallResult<(Vec<(Nat,String,)>,)> {
        ic_cdk::call(self.0, "getLog", ()).await
    }

    pub async fn get_user_icsps(&self) -> CallResult<
        (Vec<(String,Principal,)>,)
    > { ic_cdk::call(self.0, "getUserICSPs", ()).await }

    pub async fn top_up_icsp(&self, args: TopUpArgs) -> CallResult<(TopUpResult,)> {
        ic_cdk::call(self.0, "topUpICSP", (args,)).await
    }

    pub async fn top_up_self(&self, _caller: Principal) -> CallResult<()> {
        ic_cdk::call(self.0, "topUpSelf", (_caller,)).await
    }

    pub async fn transform_icp(&self, args: TransformArgs) -> CallResult<
        (TopUpResult,)
    > { ic_cdk::call(self.0, "transformIcp", (args,)).await }

    pub async fn update_icsp_wasm(&self, _wasm: Vec<u8>) -> CallResult<(String,)> {
        ic_cdk::call(self.0, "updateICSPWasm", (_wasm,)).await
    }

    pub async fn wallet_receive(&self) -> CallResult<()> {
        ic_cdk::call(self.0, "wallet_receive", ()).await
    }

}
