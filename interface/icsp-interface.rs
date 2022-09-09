use candid::{Nat, Principal};
use ic_cdk::api::call::CallResult;
use ic_cdk::export::candid::{self, CandidType, Deserialize};

#[derive(CandidType, Deserialize)]
struct LiveBucketExt {
    used_memory: Nat,
    canister_id: Principal,
}

#[derive(CandidType, Deserialize)]
struct Buckets {
    old_buckets: Vec<Principal>,
    live_buckets: Vec<LiveBucketExt>,
}

#[derive(CandidType, Deserialize)]
struct StoreArgs {
    key: String,
    value: Vec<u8>,
    total_index: Nat,
    file_type: String,
    is_http_open: bool,
    total_size: u64,
    index: Nat,
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
