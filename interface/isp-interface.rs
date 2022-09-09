use candid::{Nat, Principal};
use ic_cdk::api::call::CallResult;
use ic_cdk::export::candid::{self, CandidType, Deserialize};

#[derive(CandidType, Deserialize)]
pub enum Error {
    Create_Canister_Failed(Nat),
    Ledger_Transfer_Failed(Nat),
    Unauthorized,
}

#[derive(CandidType, Deserialize)]
pub enum CreateICSPResult {
    ok(Principal),
    err(Error),
}

#[derive(CandidType, Deserialize)]
pub struct TopUpArgs {
    icsp_canister_id: Principal,
    icp_amount: u64,
}

#[derive(CandidType, Deserialize)]
pub enum TopUpResult {
    ok,
    err(Error),
}

#[derive(CandidType, Deserialize)]
pub struct TransformArgs {
    to_canister_id: Principal,
    icp_amount: u64,
}

pub type ISP = candid::Service;
pub type AccountIdentifier = Vec<u8>;

pub struct SERVICE(pub candid::Principal);
impl SERVICE {
    pub async fn get_sub_account(&self) -> CallResult<(AccountIdentifier,)> {
        ic_cdk::call(self.0, "getSubAccount", ()).await
    }

    pub async fn create_icsp(&self, name: String, amount: u64) -> CallResult<(CreateICSPResult,)> {
        ic_cdk::call(self.0, "createICSP", (name, amount)).await
    }

    pub async fn get_admins(&self) -> CallResult<(Vec<Principal>,)> {
        ic_cdk::call(self.0, "getAdmins", ()).await
    }

    pub async fn get_user_icsps(&self) -> CallResult<(Vec<(String, Principal)>,)> {
        ic_cdk::call(self.0, "getUserICSPs", ()).await
    }

    pub async fn top_up_icsp(&self, args: TopUpArgs) -> CallResult<(TopUpResult,)> {
        ic_cdk::call(self.0, "topUpICSP", (args,)).await
    }

    pub async fn transform_icp(&self, args: TransformArgs) -> CallResult<(TopUpResult,)> {
        ic_cdk::call(self.0, "transformIcp", (args,)).await
    }
}
