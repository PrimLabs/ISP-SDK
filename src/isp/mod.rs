mod isp_did;
use candid::{CandidType, Decode, Encode, Nat};
use garcon::Delay;
use hex::{self};
use ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport;
use ic_agent::{identity::Secp256k1Identity, Agent};
pub use isp_did::{CreateICSPResult, Error, TopUpArgs, TopUpResult, TransferResult};
use serde::Deserialize;

static ISP_CANISTER_ID_TEXT: &'static str = "p2pki-xyaaa-aaaan-qatua-cai";

/// Get icsps of user, return Vec<(icsp_name, icsp_canister_id)>
///
/// # Examples
///
/// ``` no_run
/// use isp_sdk::isp;
///
/// pub async fn get_user_icsps() {
///     let response = isp::get_user_icsps("identities/identity.pem").await;
///     for i in &response {
///         println!("icsp_name:{:?},icsp_canister_id:{:?}", i.0, i.1.to_text());
///     }
///     if response.is_empty() {
///         println!("user do not have icsp\n");
///     }
/// }
/// ```
pub async fn get_user_icsps(pem_identity_path: &str) -> Vec<(String, candid::Principal)> {
    let canister_id = candid::Principal::from_text(ISP_CANISTER_ID_TEXT).unwrap();
    let response_blob = build_agent(pem_identity_path)
        .query(&canister_id, "getUserICSPs")
        .with_arg(Encode!().expect("encode error"))
        .call()
        .await
        .expect("response error");
    let response = Decode!(&response_blob, Vec<(String, candid::Principal)>).unwrap();
    response
}

/// Get user's subAccount of the isp
///
/// You should transfer icp to this subAccount in order to create icsp canister
///
/// # Examples
///
/// ``` no_run
/// use isp_sdk::isp;
///
/// pub async fn get_sub_account() {
///     println!(
///         "SubAccount:{:?}\n",
///         isp::get_sub_account("identities/identity.pem").await
///     );
/// }
/// ```
pub async fn get_sub_account(pem_identity_path: &str) -> String {
    let canister_id = candid::Principal::from_text(ISP_CANISTER_ID_TEXT).unwrap();
    let response_blob = build_agent(pem_identity_path)
        .query(&canister_id, "getSubAccount")
        .with_arg(Encode!().expect("encode error"))
        .call()
        .await
        .expect("response error");
    let response = Decode!(&response_blob, Vec<u8>).unwrap();
    hex::encode(response)
}

/// Get the icp balance of user's subAccount of the isp
///
/// The balance is e8s
///
/// # Examples
///
/// ``` no_run
/// use isp_sdk::isp;
///
/// pub async fn get_user_sub_account_icp_balance() {
///     println!(
///         "icp balance:{:?}\n",
///         isp::get_user_sub_account_icp_balance("identities/identity.pem").await
///     );
/// }
/// ```
pub async fn get_user_sub_account_icp_balance(pem_identity_path: &str) -> u64 {
    let canister_id = candid::Principal::from_text(ISP_CANISTER_ID_TEXT).unwrap();
    let response_blob = build_agent(pem_identity_path)
        .update(&canister_id, "getUserSubAccountICPBalance")
        .with_arg(Encode!().expect("encode error"))
        .call_and_wait()
        .await
        .expect("response error");
    let response = Decode!(&response_blob, u64).unwrap();
    response
}

/// Transfer out icp from user's subAccount of the isp
///
/// The amount is e8s
///
/// # Examples
///
/// ``` no_run
/// use isp_sdk::isp;
///
/// pub async fn transfer_out_user_sub_account_icp() {
///     println!(
///         "transfer out icp result:{:?}\n",
///         isp::transfer_out_user_sub_account_icp(
///             "identities/identity.pem",
///             "3eee9b4671b8fde5a501288d74d21ee93042dc202104fa35051563ae35d24f2f",
///             5000000 as u64
///         )
///         .await
///     );
/// }
/// ```
pub async fn transfer_out_user_sub_account_icp(
    pem_identity_path: &str,
    to: &str,
    amount: u64,
) -> TransferResult {
    let canister_id = candid::Principal::from_text(ISP_CANISTER_ID_TEXT).unwrap();
    let response_blob = build_agent(pem_identity_path)
        .update(&canister_id, "transferOutUserSubAccountICP")
        .with_arg(Encode!(&(hex::decode(to).unwrap()), &amount).expect("encode error"))
        .call_and_wait()
        .await
        .expect("response error");
    let response = Decode!(&response_blob, TransferResult).unwrap();
    response
}

/// Get admins of isp
///
/// # Examples
///
/// ``` no_run
/// use candid::Principal;
/// use isp_sdk::isp;
///
/// pub async fn get_isp_admins() {
///     println!("isp admins:");
///     for i in &isp::get_isp_admins("identities/identity.pem").await {
///         println!("{:?}", Principal::to_text(i));
///     }
/// }
/// ```
pub async fn get_isp_admins(pem_identity_path: &str) -> Vec<candid::Principal> {
    let canister_id = candid::Principal::from_text(ISP_CANISTER_ID_TEXT).unwrap();
    let response_blob = build_agent(pem_identity_path)
        .query(&canister_id, "getAdmins")
        .with_arg(Encode!().expect("encode error"))
        .call()
        .await
        .expect("response error");
    let response = Decode!(&response_blob, Vec<candid::Principal>).unwrap();
    response
}

/// Get ISP's ICSP_WASM version
///
/// # Examples
///
/// ``` no_run
/// use isp_sdk::isp;
///
/// pub async fn get_isp_version() {
///     println!(
///         "isp version: {:?}",
///         isp::get_version("identities/identity.pem").await
///     );
/// }
/// ```
pub async fn get_version(pem_identity_path: &str) -> String {
    let canister_id = candid::Principal::from_text(ISP_CANISTER_ID_TEXT).unwrap();
    let response_blob = build_agent(pem_identity_path)
        .query(&canister_id, "getVersion")
        .with_arg(Encode!().expect("encode error"))
        .call()
        .await
        .expect("response error");
    Decode!(&response_blob, String).unwrap()
}

/// Use icp to create a icsp canister and use the [XTC](https://github.com/Psychedelic/dank/tree/main/xtc) to top_up it
///
/// You must ensure that your subAccount has sufficient icp
///
/// And your pem Account have sufficient [XTC](https://github.com/Psychedelic/dank/tree/main/xtc)
///
/// Notice:
///
/// The icp_amount is e8s.
/// 1 icp should be 100_000_000
///
/// The XTC is e12s.
/// 1 T XTC(Cycles) should be 1_000_000_000_000
///
/// # Examples
///
/// ``` no_run
/// use isp_sdk::isp::{self, CreateICSPResult};
///
/// pub async fn create_icsp() {
///     let response = isp::create_icsp(
///         "identities/identity.pem",
///         "icsp-1",
///         15_000_000 as u64,
///         5_000_000_000_000 as u64 - 2_000_000_000 as u64,
///     )
///         .await;
///     match response.0 {
///         CreateICSPResult::ok(canister_id) => {
///             println!("create icsp success: {:?}", canister_id.to_text());
///             println!("use XTC topup result: {:?}", response.1.unwrap());
///         }
///         CreateICSPResult::err(error) => {
///             println!("create icsp error: {:?}", error);
///         }
///     }
/// }
/// ```
pub async fn create_icsp(
    pem_identity_path: &str,
    icsp_name: &str,
    icp_to_create_amount: u64,
    xtc_to_topup_amount: u64,
) -> (CreateICSPResult, Option<BurnResult>) {
    // create a icsp canister
    let isp_canister_id = candid::Principal::from_text(ISP_CANISTER_ID_TEXT).unwrap();
    let agent = build_agent(pem_identity_path);
    let response_blob = agent
        .update(&isp_canister_id, "createICSP")
        .with_arg(Encode!(&icsp_name, &icp_to_create_amount).expect("encode error"))
        .call_and_wait()
        .await
        .expect("response error");
    let response = Decode!(&response_blob, CreateICSPResult).unwrap();
    match response {
        CreateICSPResult::ok(icsp_canister_id) => {
            // use XTC to topup icsp
            let top_up_response = top_up_icsp_with_xtc(
                pem_identity_path,
                BurnArgs {
                    canister_id: icsp_canister_id,
                    amount: xtc_to_topup_amount,
                },
            )
            .await;
            match top_up_response {
                BurnResult::Ok(block_index) => {
                    // init icsp
                    let _init_response = agent
                        .update(
                            &candid::Principal::from_text(icsp_canister_id.to_text()).unwrap(),
                            "init",
                        )
                        .with_arg(Encode!().expect("encode error"))
                        .call_and_wait()
                        .await
                        .expect("response error");
                    return (
                        CreateICSPResult::ok(icsp_canister_id),
                        Some(BurnResult::Ok(block_index)),
                    );
                }
                BurnResult::Err(burn_err) => {
                    return (
                        CreateICSPResult::ok(icsp_canister_id),
                        Some(BurnResult::Err(burn_err)),
                    );
                }
            }
        }
        CreateICSPResult::err(create_err) => {
            return (CreateICSPResult::err(create_err), None);
        }
    }
}

/// Transform icp to cycles and top_up tp icsp
///
/// # Examples
///
/// ``` no_run
/// use candid::Principal;
/// use isp_sdk::isp::{self, TopUpArgs};
///
/// pub async fn top_up_icsp() {
///   println!(
///      "topup icsp result:{:?}\n",
///      isp::top_up_icsp(
///          "identities/identity.pem",
///          TopUpArgs {
///             icsp_canisterId: Principal::from_text("xk2my-yqaaa-aaaal-abdwa-cai").unwrap(),
///              icp_amount: 5_000_000 as u64,
///          }
///      )
///           .await
///   );
/// }
/// ```
pub async fn top_up_icsp(pem_identity_path: &str, args: TopUpArgs) -> TopUpResult {
    let canister_id = candid::Principal::from_text(ISP_CANISTER_ID_TEXT).unwrap();
    let response_blob = build_agent(pem_identity_path)
        .update(&canister_id, "topUpICSP")
        .with_arg(Encode!(&args).expect("encode error"))
        .call_and_wait()
        .await
        .expect("response error");
    let response = Decode!(&response_blob, TopUpResult).unwrap();
    response
}

#[derive(CandidType, Deserialize, Debug)]
pub struct BurnArgs {
    pub canister_id: candid::Principal,
    pub amount: u64,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum BurnResult {
    Ok(u64),
    Err(BurnError),
}

#[derive(CandidType, Deserialize, Debug)]
pub enum BurnError {
    InsufficientBalance,
    InvalidTokenContract,
    NotSufficientLiquidity,
}

/// Use [XTC](https://github.com/Psychedelic/dank/tree/main/xtc) to top_up icsp
///
/// You must ensure you pem Account have sufficient [XTC](https://github.com/Psychedelic/dank/tree/main/xtc)
///
/// # Examples
///
/// ``` no_run
/// use isp_sdk::isp::{self, BurnArgs};
/// use candid::Principal;
///
/// pub async fn top_up_icsp_with_xtc() {
///   println!(
///       "topup icsp with XTC result:{:?}\n",
///      isp::top_up_icsp_with_xtc(
///         "identities/identity.pem",
///         BurnArgs {
///             canister_id: Principal::from_text("hf34l-eyaaa-aaaan-qav5q-cai").unwrap(),
///            amount: 1_000_000_000_000 as u64 - 2_000_000_000 as u64,
///         }
///     )
///          .await
///  );
/// }
/// ```
pub async fn top_up_icsp_with_xtc(pem_identity_path: &str, args: BurnArgs) -> BurnResult {
    let canister_id = candid::Principal::from_text("aanaa-xaaaa-aaaah-aaeiq-cai").unwrap();
    let response_blob = build_agent(pem_identity_path)
        .update(&canister_id, "burn")
        .with_arg(Encode!(&args).expect("encode error"))
        .call_and_wait()
        .await
        .expect("response error");
    let response = Decode!(&response_blob, BurnResult).unwrap();
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
