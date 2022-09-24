// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
use ic_cdk::api::call::CallResult;
use ic_cdk::export::candid::{self, CandidType, Deserialize};

#[derive(CandidType, Deserialize)]
struct LiveBucketExt__1 {
    used_memory: candid::Nat,
    canister_id: candid::Principal,
}

#[derive(CandidType, Deserialize)]
struct Buckets {
    old_buckets: Vec<candid::Principal>,
    live_buckets: Vec<LiveBucketExt__1>,
}

#[derive(CandidType, Deserialize)]
enum FileLocation {
    IPFS,
    Arweave,
}

#[derive(CandidType, Deserialize)]
struct HeaderField(String, String);

#[derive(CandidType, Deserialize)]
struct HttpRequest {
    url: String,
    method: String,
    body: Vec<u8>,
    headers: Vec<HeaderField>,
}

#[derive(CandidType, Deserialize)]
struct CallbackToken {
    key: String,
    total_index: candid::Nat,
    index: candid::Nat,
}

#[derive(CandidType, Deserialize)]
struct StreamingCallbackHttpResponse {
    token: Option<CallbackToken>,
    body: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
enum StreamStrategy {
    Callback {
        token: CallbackToken,
        callback: candid::Func,
    },
}

#[derive(CandidType, Deserialize)]
struct HttpResponse {
    body: Vec<u8>,
    headers: Vec<HeaderField>,
    streaming_strategy: Option<StreamStrategy>,
    status_code: u16,
}

#[derive(CandidType, Deserialize)]
struct LiveBucketExt {
    used_memory: candid::Nat,
    canister_id: candid::Principal,
}

#[derive(CandidType, Deserialize)]
struct StoreArgs {
    key: String,
    value: Vec<u8>,
    total_index: candid::Nat,
    file_type: String,
    is_http_open: bool,
    total_size: u64,
    index: candid::Nat,
}

type icsp = candid::Service;
struct SERVICE(candid::Principal);
impl SERVICE {
    pub async fn getAdmins(&self) -> CallResult<(Vec<candid::Principal>,)> {
        ic_cdk::call(self.0, "getAdmins", ()).await
    }

    pub async fn getBucketOfFile(&self, arg0: String) -> CallResult<(Option<candid::Principal>,)> {
        ic_cdk::call(self.0, "getBucketOfFile", (arg0,)).await
    }
    pub async fn getBuckets(&self) -> CallResult<(Option<Buckets>,)> {
        ic_cdk::call(self.0, "getBuckets", ()).await
    }

    pub async fn http_request(&self, arg0: HttpRequest) -> CallResult<(HttpResponse,)> {
        ic_cdk::call(self.0, "http_request", (arg0,)).await
    }

    pub async fn store(&self, arg0: StoreArgs) -> CallResult<()> {
        ic_cdk::call(self.0, "store", (arg0,)).await
    }

    pub async fn wallet_receive(&self) -> CallResult<(candid::Nat,)> {
        ic_cdk::call(self.0, "wallet_receive", ()).await
    }
}
