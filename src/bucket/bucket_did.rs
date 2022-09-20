use candid::{Nat, Principal};
use ic_cdk::api::call::CallResult;
use ic_cdk::export::candid::{self, CandidType, Deserialize};

type Bucket = candid::Service;

struct SERVICE(candid::Principal);
impl SERVICE {
    pub async fn add_admin(&self, new_admin: Principal) -> CallResult<(bool,)> {
        ic_cdk::call(self.0, "addAdmin", (new_admin,)).await
    }

    pub async fn change_admin(&self, new_admins: Vec<Principal>) -> CallResult<(bool,)> {
        ic_cdk::call(self.0, "changeAdmin", (new_admins,)).await
    }

    pub async fn get(&self, key: String, index: Nat) -> CallResult<(Option<(Vec<u8>, String)>,)> {
        ic_cdk::call(self.0, "get", (key, index)).await
    }

    pub async fn get_admins(&self) -> CallResult<(Vec<Principal>,)> {
        ic_cdk::call(self.0, "getAdmins", ()).await
    }
    pub async fn get_assets(
        &self,
    ) -> CallResult<(Vec<(String, (Vec<(u64, u64)>, String, Nat, bool))>,)> {
        ic_cdk::call(self.0, "getAssets", ()).await
    }

    pub async fn get_buffers(&self) -> CallResult<(Vec<String>,)> {
        ic_cdk::call(self.0, "getBuffers", ()).await
    }

    pub async fn get_file_total_index(&self, key: String) -> CallResult<(Nat,)> {
        ic_cdk::call(self.0, "getFileTotalIndex", (key,)).await
    }
}
