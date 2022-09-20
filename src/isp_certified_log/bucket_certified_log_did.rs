use candid::Principal;
use ic_cdk::api::call::CallResult;
use ic_cdk::export::candid::{self, CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug)]
pub struct CertifiedLog {
    context: String,
    cert: Vec<u8>,
    time: u64,
    witness: Vec<u8>,
    index: u64,
}

#[derive(CandidType, Deserialize)]
pub struct Log {
    context: String,
}

struct SERVICE(Principal);
impl SERVICE {
    pub async fn add_admin(&self, new_admin: Principal) -> CallResult<(bool,)> {
        ic_cdk::call(self.0, "addAdmin", (new_admin,)).await
    }
    pub async fn change_admin(&self, new_admin: Vec<Principal>) -> CallResult<(bool,)> {
        ic_cdk::call(self.0, "changeAdmin", (new_admin,)).await
    }
    pub async fn get_logs(&self, start: u64, end: u64) -> CallResult<(Option<Vec<CertifiedLog>>,)> {
        ic_cdk::call(self.0, "getLogs", (start, end)).await
    }
    pub async fn put(&self, log: Log) -> CallResult<()> {
        ic_cdk::call(self.0, "put", (log,)).await
    }
}
