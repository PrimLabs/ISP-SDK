mod isp_did;
use candid::{CandidType, Decode, Encode, Nat};
use garcon::Delay;
use hex::{self};
use ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport;
use ic_agent::{ic_types::Principal, identity::Secp256k1Identity, Agent};
pub use isp_did::{CreateICSPResult, Error, TopUpArgs, TopUpResult};
use serde::Deserialize;

static ISP_CANISTER_ID_TEXT: &'static str = "p2pki-xyaaa-aaaan-qatua-cai";

/// Get icsps of user, return Vec<(icsp_name, icsp_canister_id)>
///
/// Example code :
/// ``` no_run
/// use ic_agent::ic_types::Principal;
/// use isp_sdk::isp;
///
/// async fn get_user_icsps() -> Vec<(String, Principal)> {
///     isp::get_user_icsps("identities/identity.pem").await
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let response = get_user_icsps().await;
///     for i in &response {
///         println!("icsp_name:{:?},icsp_canister_id:{:?}", i.0, i.1.to_text());
///     }
///     if response.is_empty() {
///         println!("user do not have icsp\n");
///     }
/// }
/// ```
pub async fn get_user_icsps(pem_identity_path: &str) -> Vec<(String, Principal)> {
    let canister_id = Principal::from_text(ISP_CANISTER_ID_TEXT).unwrap();
    let agent = build_agent(pem_identity_path);
    let response_blob = agent
        .query(&canister_id, "getUserICSPs")
        .with_arg(Encode!().expect("encode error"))
        .call()
        .await
        .expect("response error");
    let response = Decode!(&response_blob, Vec<(String, Principal)>).unwrap();
    response
}

/// Get user's subAccount of the isp
///
/// You should transfer icp to this subAccount in order to create icsp
///
/// The icp is transformed into cycles which are topuped to icsp
///
/// Example code :
/// ``` no_run
/// use isp_sdk::isp;
///
/// async fn get_sub_account() -> String {
///     isp::get_sub_account("identities/identity.pem").await
/// }
///
/// #[tokio::main]
/// async fn main() {
///     println!("subAccount:{:?}\n", get_sub_account());
/// }
/// ```
pub async fn get_sub_account(pem_identity_path: &str) -> String {
    let canister_id = Principal::from_text(ISP_CANISTER_ID_TEXT).unwrap();
    let agent = build_agent(pem_identity_path);
    let response_blob = agent
        .query(&canister_id, "getSubAccount")
        .with_arg(Encode!().expect("encode error"))
        .call()
        .await
        .expect("response error");
    let response = Decode!(&response_blob, Vec<u8>).unwrap();
    hex::encode(response)
}

/// Get admins of isp
///
/// Example code :
/// ``` no_run
/// use ic_agent::ic_types::Principal;
/// use isp_sdk::isp;
///
/// async fn get_isp_admins() -> Vec<Principal> {
///     isp::get_isp_admins("identities/identity.pem").await
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let response = get_isp_admins().await;
///     println!("isp admins:");
///     for i in &response {
///         println!("{:?}", Principal::to_text(i));
///     }
/// }
/// ```
pub async fn get_isp_admins(pem_identity_path: &str) -> Vec<Principal> {
    let canister_id = Principal::from_text(ISP_CANISTER_ID_TEXT).unwrap();
    let agent = build_agent(pem_identity_path);
    let response_blob = agent
        .query(&canister_id, "getAdmins")
        .with_arg(Encode!().expect("encode error"))
        .call()
        .await
        .expect("response error");
    let response = Decode!(&response_blob, Vec<Principal>).unwrap();
    response
}

/// Create a icsp
///
/// You must ensure that your subAccount has sufficient icp
///
/// Notice: the icp_amount is e8s.
///
/// 1 icp should be 100_000_000
///
/// Example code :
/// ``` no_run
/// use isp_sdk::isp::{self, CreateICSPResult};
///
/// async fn create_icsp(icsp_name: &str, icp_amount: u64) -> CreateICSPResult {
///     isp::create_icsp("identities/identity.pem", icsp_name, icp_amount).await
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let response = create_icsp("icsp-1", 100_000_000 as u64).await;
///     println!("create icsp result:{:?}\n", response);
/// }
/// ```
pub async fn create_icsp(
    pem_identity_path: &str,
    icsp_name: &str,
    icp_amount: u64,
) -> CreateICSPResult {
    let canister_id = Principal::from_text(ISP_CANISTER_ID_TEXT).unwrap();
    let agent = build_agent(pem_identity_path);
    let waiter = get_waiter();
    let response_blob = agent
        .update(&canister_id, "createICSP")
        .with_arg(Encode!(&icsp_name, &icp_amount).expect("encode error"))
        .call_and_wait(waiter)
        .await
        .expect("response error");
    let response = Decode!(&response_blob, CreateICSPResult).unwrap();
    response
}

/// Transform icp to cycles and topup tp icsp
///
/// Example code :
/// ``` no_run
/// use ic_agent::ic_types::Principal;
/// use isp_sdk::isp::{self, TopUpArgs, TopUpResult};
///
/// async fn top_up_icsp(args: TopUpArgs) -> TopUpResult {
///     isp::top_up_icsp("identities/identity.pem", args).await
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let top_up_args = TopUpArgs {
///         icsp_canisterId: Principal::from_text("xk2my-yqaaa-aaaal-abdwa-cai").unwrap(),
///         icp_amount: 5_000_000 as u64,
///     };
///     let response = top_up_icsp(top_up_args).await;
///     println!("topup icsp result:{:?}\n", response);
/// }
pub async fn top_up_icsp(pem_identity_path: &str, args: TopUpArgs) -> TopUpResult {
    let canister_id = Principal::from_text(ISP_CANISTER_ID_TEXT).unwrap();
    let agent = build_agent(pem_identity_path);
    let waiter = get_waiter();
    let response_blob = agent
        .update(&canister_id, "topUpICSP")
        .with_arg(Encode!(&args).expect("encode error"))
        .call_and_wait(waiter)
        .await
        .expect("response error");
    let response = Decode!(&response_blob, TopUpResult).unwrap();
    response
}

fn get_waiter() -> Delay {
    let waiter = garcon::Delay::builder()
        .throttle(std::time::Duration::from_millis(500))
        .timeout(std::time::Duration::from_secs(60 * 5))
        .build();
    waiter
}

fn build_agent(pem_identity_path: &str) -> Agent {
    let url = "https://ic0.app".to_string();
    let identity = Secp256k1Identity::from_pem_file(String::from(pem_identity_path)).unwrap();
    let transport = ReqwestHttpReplicaV2Transport::create(url).expect("transport error");
    let agent = Agent::builder()
        .with_transport(transport)
        .with_identity(identity)
        .build()
        .expect("build agent error");
    agent
}
