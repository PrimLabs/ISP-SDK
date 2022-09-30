use candid::{Nat, Principal};
use ic_cdk::api::call::CallResult;
use ic_cdk::export::candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug)]
pub struct LiveBucket {
    pub bucket_id: String,
    pub used_memory: Nat,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct Buckets {
    pub old_buckets: Vec<Principal>,
    pub live_buckets: LiveBucket,
}

#[derive(CandidType, Deserialize)]
pub struct StoreLog {
    pub context: String,
}

struct SERVICE(Principal);
impl SERVICE {
    pub async fn add_admin(&self, new_admin: Principal) -> CallResult<()> {
        ic_cdk::call(self.0, "addAdmin", (new_admin,)).await
    }
    pub async fn delete_admin(&self, old_admin: Principal) -> CallResult<()> {
        ic_cdk::call(self.0, "deleteAdmin", (old_admin,)).await
    }
    pub async fn get_admins(&self) -> CallResult<(Vec<Principal>,)> {
        ic_cdk::call(self.0, "getAdmins", ()).await
    }
    pub async fn get_buckets(&self) -> CallResult<(Option<Buckets>,)> {
        ic_cdk::call(self.0, "getBuckets", ()).await
    }
    pub async fn get_log_num(&self) -> CallResult<(Nat,)> {
        ic_cdk::call(self.0, "getLogNum", ()).await
    }
    pub async fn get_logs(
        &self,
        start: Nat,
        end: Nat,
    ) -> CallResult<(Option<Vec<(u64, u64, Principal)>>,)> {
        ic_cdk::call(self.0, "getLogs", (start, end)).await
    }
    pub async fn init(&self) -> CallResult<(LiveBucket,)> {
        ic_cdk::call(self.0, "init", ()).await
    }
    pub async fn store(&self, args: StoreLog) -> CallResult<()> {
        ic_cdk::call(self.0, "store", (args,)).await
    }
    pub async fn top_up_bucket(&self, amount: Nat) -> CallResult<()> {
        ic_cdk::call(self.0, "topUpBucket", (amount,)).await
    }
    pub async fn update_bucket_canister_controller(
        &self,
        canister_id: ic_cdk::export::Principal,
        contoller: Vec<ic_cdk::export::Principal>,
    ) -> CallResult<(bool,)> {
        ic_cdk::call(
            self.0,
            "updateBucketCanisterController",
            (canister_id, contoller),
        )
        .await
    }
}
