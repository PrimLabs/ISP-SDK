use candid::{CandidType, Decode, Encode, Nat};
use garcon::Delay;
use hex::{self};
use ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport;
use ic_agent::{ic_types::Principal, identity::Secp256k1Identity, Agent};
use serde::Deserialize;

#[derive(CandidType, Deserialize, Debug)]
pub enum Error {
    Create_Canister_Failed(Nat),
    Ledger_Transfer_Failed(Nat),
    Unauthorized,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum CreateICSPResult {
    ok(Principal),
    err(Error),
}

#[derive(CandidType, Deserialize, Debug)]
pub enum TopUpResult {
    ok,
    err(Error),
}

#[derive(CandidType, Deserialize)]
pub struct TopUpArgs {
    pub icsp_canisterId: Principal,
    pub icp_amount: u64,
}

static ISP_CANISTER_ID_TEXT: &'static str = "p2pki-xyaaa-aaaan-qatua-cai";

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
