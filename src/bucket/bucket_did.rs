// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
use ic_cdk::api::call::CallResult;
use ic_cdk::export::candid::{self, CandidType, Deserialize};

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
struct CallbackToken__1 {
    key: String,
    total_index: candid::Nat,
    index: candid::Nat,
}

#[derive(CandidType, Deserialize)]
struct StreamingCallbackHttpResponse__1 {
    token: Option<CallbackToken__1>,
    body: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
enum StreamStrategy {
    Callback {
        token: CallbackToken__1,
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
struct StoreArgs {
    key: String,
    value: Vec<u8>,
    total_index: candid::Nat,
    file_type: String,
    is_http_open: bool,
    total_size: u64,
    index: candid::Nat,
}

#[derive(CandidType, Deserialize)]
struct CallbackToken {
    key: String,
    total_index: candid::Nat,
    index: candid::Nat,
}

#[derive(CandidType, Deserialize)]
struct StreamingCallbackHttpResponse {
    token: Option<CallbackToken__1>,
    body: Vec<u8>,
}

type Bucket = candid::Service;
struct SERVICE(candid::Principal);
impl SERVICE {
    pub async fn addAdmin(&self, arg0: candid::Principal) -> CallResult<(bool,)> {
        ic_cdk::call(self.0, "addAdmin", (arg0,)).await
    }
    pub async fn changeAdmin(&self, arg0: Vec<candid::Principal>) -> CallResult<(bool,)> {
        ic_cdk::call(self.0, "changeAdmin", (arg0,)).await
    }
    pub async fn delete(&self, arg0: String) -> CallResult<(bool,)> {
        ic_cdk::call(self.0, "delete", (arg0,)).await
    }
    pub async fn get(
        &self,
        arg0: String,
        arg1: candid::Nat,
    ) -> CallResult<(Option<(Vec<u8>, String)>,)> {
        ic_cdk::call(self.0, "get", (arg0, arg1)).await
    }
    pub async fn getAdmins(&self) -> CallResult<(Vec<candid::Principal>,)> {
        ic_cdk::call(self.0, "getAdmins", ()).await
    }
    pub async fn getBuffers(&self) -> CallResult<(Vec<String>,)> {
        ic_cdk::call(self.0, "getBuffers", ()).await
    }
    pub async fn getCycleBalance(&self) -> CallResult<(candid::Nat,)> {
        ic_cdk::call(self.0, "getCycleBalance", ()).await
    }
    pub async fn getFileTotalIndex(&self, arg0: String) -> CallResult<(candid::Nat,)> {
        ic_cdk::call(self.0, "getFileTotalIndex", (arg0,)).await
    }
    pub async fn http_request(&self, arg0: HttpRequest) -> CallResult<(HttpResponse,)> {
        ic_cdk::call(self.0, "http_request", (arg0,)).await
    }
    pub async fn store(&self, arg0: StoreArgs) -> CallResult<()> {
        ic_cdk::call(self.0, "store", (arg0,)).await
    }
    pub async fn streamingCallback(
        &self,
        arg0: CallbackToken,
    ) -> CallResult<(StreamingCallbackHttpResponse,)> {
        ic_cdk::call(self.0, "streamingCallback", (arg0,)).await
    }
    pub async fn wallet_receive(&self) -> CallResult<(candid::Nat,)> {
        ic_cdk::call(self.0, "wallet_receive", ()).await
    }
}
