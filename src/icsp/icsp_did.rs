use candid::{Nat, Principal};
use ic_cdk::api::call::CallResult;
use ic_cdk::export::candid::{CandidType, Deserialize};

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

#[derive(CandidType, Deserialize, Debug)]
pub struct FileBufExt {
    pub bucket_id: Principal,
    pub total_index: Nat,
    pub wrote_page: Vec<bool>,
    pub file_type: String,
    pub is_http_open: bool,
    pub total_size: u64,
    pub received: Nat,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct StoreArgs {
    pub key: String,
    pub value: Vec<u8>,
    pub total_index: Nat,
    pub file_type: String,
    pub is_http_open: bool,
    pub total_size: u64,
    pub index: Nat,
}

#[derive(CandidType, Deserialize, Debug)]
struct OtherFile {
    file_location: FileLocation,
    file_key: String,
    file_url: String,
    file_type: String,
}

#[derive(CandidType, Deserialize, Debug)]
enum FileLocation {
    IPFS,
    Arweave,
}

type icsp = candid::Service;

struct SERVICE(Principal);
impl SERVICE {
    pub async fn get_cycle_balance(&self) -> CallResult<(Nat,)> {
        ic_cdk::call(self.0, "getCycleBalance", ()).await
    }

    pub async fn get_file_info(&self, file_key: String) -> CallResult<(Option<FileBufExt>,)> {
        ic_cdk::call(self.0, "getFileInfo", (file_key,)).await
    }

    pub async fn get_all_ic_file_key(&self) -> CallResult<(Vec<String>,)> {
        ic_cdk::call(self.0, "getAllIcFileKey", ()).await
    }

    pub async fn get_all_ipfs_file_key(&self) -> CallResult<(Vec<String>,)> {
        ic_cdk::call(self.0, "getAllIpfsFileKey", ()).await
    }

    pub async fn get_all_arweave_file_key(&self) -> CallResult<(Vec<String>,)> {
        ic_cdk::call(self.0, "getAllArFileKey", ()).await
    }

    pub async fn record_file(&self, other_file: OtherFile) -> CallResult<()> {
        ic_cdk::call(self.0, "recordFile", (other_file,)).await
    }

    pub async fn init(&self) -> CallResult<(LiveBucketExt,)> {
        ic_cdk::call(self.0, "init", ()).await
    }

    pub async fn get_other_file(
        &self,
        file_key: String,
        file_location: FileLocation,
    ) -> CallResult<(Option<OtherFile>,)> {
        ic_cdk::call(self.0, "getOtherFile", (file_key, file_location)).await
    }

    pub async fn add_admin(&self, new_admin: Principal) -> CallResult<()> {
        ic_cdk::call(self.0, "addAdmin", (new_admin,)).await
    }

    pub async fn delete_admin(&self, old_admin: Principal) -> CallResult<()> {
        ic_cdk::call(self.0, "deleteAdmin", (old_admin,)).await
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
