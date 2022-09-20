use candid::{Nat, Principal};
use ic_cdk::api::call::CallResult;
use ic_cdk::export::candid::{self, CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug)]
pub struct LiveBucketExt {
    pub used_memory: Nat,
    pub canister_id: Principal,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct Buckets {
    pub old_buckets: Vec<Principal>,
    pub live_buckets: Vec<LiveBucketExt>,
}

#[derive(CandidType, Deserialize)]
pub struct StoreArgs {
    pub key: String,
    pub value: Vec<u8>,
    pub total_index: Nat,
    pub file_type: String,
    pub is_http_open: bool,
    pub total_size: u64,
    pub index: Nat,
}

type icsp = candid::Service;

struct SERVICE(Principal);
impl SERVICE {
    pub async fn add_admin(&self, new_admin: Principal) -> CallResult<(bool,)> {
        ic_cdk::call(self.0, "addAdmin", (new_admin,)).await
    }

    pub async fn change_admin(&self, new_admin: Vec<Principal>) -> CallResult<(bool,)> {
        ic_cdk::call(self.0, "changeAdmin", (new_admin,)).await
    }

    pub async fn change_bucket_admin(&self) -> CallResult<(bool,)> {
        ic_cdk::call(self.0, "change_bucket_admin", ()).await
    }

    pub async fn get_admins(&self) -> CallResult<(Vec<Principal>,)> {
        ic_cdk::call(self.0, "getAdmins", ()).await
    }

    pub async fn get_bucket_of_file(&self, key: String) -> CallResult<(Option<Principal>,)> {
        ic_cdk::call(self.0, "getBucketOfFile", (key,)).await
    }

    pub async fn get_buckets(&self) -> CallResult<(Option<Buckets>,)> {
        ic_cdk::call(self.0, "getBuckets", ()).await
    }

    pub async fn store(&self, args: StoreArgs) -> CallResult<()> {
        ic_cdk::call(self.0, "store", (args,)).await
    }
}
