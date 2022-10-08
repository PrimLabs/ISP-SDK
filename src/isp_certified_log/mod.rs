mod bucket_certified_log_did;
mod icsp_certified_log_backend_did;
pub use bucket_certified_log_did::CertifiedLog;
use candid::{CandidType, Decode, Encode, Nat};
use garcon::Delay;
use ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport;
use ic_agent::{identity::Secp256k1Identity, Agent};
pub use icsp_certified_log_backend_did::{Buckets, StoreLog};

/// Get buckets of user's icsp_certified_log
///
/// Example code :
/// ```no_run
/// use isp_sdk::isp_certified_log::{self, Buckets};
///
/// async fn get_buckets(icsp_log_canister_id_text: &str) -> Option<Buckets> {
///     isp_certified_log::get_buckets("identities/identity.pem", icsp_log_canister_id_text).await
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let response = get_buckets("4radi-oqaaa-aaaan-qapwa-cai").await;
///     match response {
///         Some(response) => {
///             println!("old buckets:");
///             for i in &response.old_buckets {
///                 println!("{:?}", i.to_text());
///             }
///             println!("Live Buckets:");
///             println!("canister_id:{:?}, used_memory:{:?}",response.live_buckets.bucket_id, response.live_buckets.used_memory,);
///         }
///         None => println!("icsp do not have buckets"),
///     }
/// }
/// ```
pub async fn get_buckets(
    pem_identity_path: &str,
    icsp_log_canister_id_text: &str,
) -> Option<Buckets> {
    let canister_id = candid::Principal::from_text(icsp_log_canister_id_text).unwrap();
    let response_blob = build_agent(pem_identity_path)
        .query(&canister_id, "getBuckets")
        .with_arg(Encode!().expect("encode piece failed"))
        .call()
        .await
        .expect("response error");
    let response = Decode!(&response_blob, Option<Buckets>).unwrap();
    response
}

/// Get the number of certified logs
///
/// Example code :
/// ```no_run
/// use isp_sdk::isp_certified_log;
///
/// async fn get_log_num(icsp_log_canister_id_text: &str) -> u128 {
///     isp_certified_log::get_log_num("identities/identity.pem", icsp_log_canister_id_text).await
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let response = get_log_num("4radi-oqaaa-aaaan-qapwa-cai").await;
///     println!("log num:{:?}", response);
/// }
/// ```
pub async fn get_log_num(pem_identity_path: &str, icsp_log_canister_id_text: &str) -> u128 {
    let canister_id = candid::Principal::from_text(icsp_log_canister_id_text).unwrap();
    let response_blob = build_agent(pem_identity_path)
        .query(&canister_id, "getLogNum")
        .with_arg(Encode!().expect("encode piece failed"))
        .call()
        .await
        .expect("response error");
    let response = Decode!(&response_blob, u128).unwrap();
    response
}

/// Get logs from start to end (0 ... n-1)
///
/// Example code :
/// ```no_run
/// use isp_sdk::isp_certified_log::{self, CertifiedLog};
///
/// async fn get_logs(icsp_log_canister_id_text: &str, start: u128, end: u128) -> Option<Vec<CertifiedLog>> {
///     isp_certified_log::get_logs(
///         "identities/identity.pem",
///         icsp_log_canister_id_text,
///         start,
///         end,
///     ).await
/// }
///
/// async fn get_log_num(icsp_log_canister_id_text: &str) -> u128 {
///     isp_certified_log::get_log_num("identities/identity.pem", icsp_log_canister_id_text).await
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let response = get_logs("4radi-oqaaa-aaaan-qapwa-cai", 0, get_log_num("4radi-oqaaa-aaaan-qapwa-cai").await - 1).await;
///     match response {
///         Some(response) => println!("{:?}", response),
///         None => println!("no logs"),
///     }
/// }
/// ```
pub async fn get_logs(
    pem_identity_path: &str,
    icsp_log_canister_id_text: &str,
    start: u128,
    end: u128,
) -> Option<Vec<CertifiedLog>> {
    let canister_id = candid::Principal::from_text(icsp_log_canister_id_text).unwrap();
    let agent = build_agent(pem_identity_path);
    let response_blob = agent
        .query(&canister_id, "getLogs")
        .with_arg(Encode!(&start, &end).expect("encode piece failed"))
        .call()
        .await
        .expect("response error");
    let response = Decode!(&response_blob, Option<Vec<(u64, u64, candid::Principal)>>).unwrap();
    match response {
        Some(ans) => {
            let mut payload: Vec<CertifiedLog> = Vec::new();
            for i in &ans {
                let response_blob = agent
                    .query(
                        &candid::Principal::from_text(i.2.to_text()).unwrap(),
                        "getLogs",
                    )
                    .with_arg(Encode!(&i.0, &i.1).expect("encode piece failed"))
                    .call()
                    .await
                    .expect("response error");
                let response = Decode!(&response_blob, Option<Vec<CertifiedLog>>).unwrap();
                match response {
                    Some(mut log) => {
                        payload.append(&mut log);
                    }
                    None => {}
                }
            }
            if payload.is_empty() {
                return None;
            } else {
                return Some(payload);
            }
        }
        None => return None,
    }
}

/// Get icsp_certified_log admins
///
/// Example code :
/// ```no_run
/// use isp_sdk::isp_certified_log;
/// use candid::Principal;
///
/// async fn get_admins(icsp_log_canister_id_text: &str) -> Vec<Principal> {
///     isp_certified_log::get_admins("identities/identity.pem", icsp_log_canister_id_text).await
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let response = get_admins("4radi-oqaaa-aaaan-qapwa-cai").await;
///     println!("admins");
///     for i in &response {
///         println!("{:?}", i.to_text());
///     }
/// }
/// ```
pub async fn get_admins(
    pem_identity_path: &str,
    icsp_log_canister_id_text: &str,
) -> Vec<candid::Principal> {
    let canister_id = candid::Principal::from_text(icsp_log_canister_id_text).unwrap();
    let response_blob = build_agent(pem_identity_path)
        .query(&canister_id, "getAdmins")
        .with_arg(Encode!().expect("encode piece failed"))
        .call()
        .await
        .expect("response error");
    let response = Decode!(&response_blob, Vec<candid::Principal>).unwrap();
    response
}

/// Store a certified log
///
/// Example code :
/// ```no_run
/// use isp_sdk::isp_certified_log::{self, StoreLog};
///
/// async fn store(icsp_log_canister_id_text: &str, args: StoreLog) {
///     isp_certified_log::store("identities/identity.pem", icsp_log_canister_id_text, args).await
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let response = store(
///         "4radi-oqaaa-aaaan-qapwa-cai",
///         StoreLog {
///             context: "test".to_string(),
///         }
///     ).await;
/// }
/// ```
pub async fn store(pem_identity_path: &str, icsp_log_canister_id_text: &str, args: StoreLog) {
    let canister_id = candid::Principal::from_text(icsp_log_canister_id_text).unwrap();
    let response_blob = build_agent(pem_identity_path)
        .update(&canister_id, "store")
        .with_arg(Encode!(&args).expect("encode piece failed"))
        .call_and_wait(get_waiter())
        .await
        .expect("response error");
    let response = Decode!(&response_blob, ()).unwrap();
    response
}

pub async fn update_bucket_canister_controller(
    pem_identity_path: &str,
    icsp_log_canister_id_text: &str,
    bucket_canister_id: candid::Principal,
    contoller: Vec<candid::Principal>,
) -> bool {
    let canister_id = candid::Principal::from_text(icsp_log_canister_id_text).unwrap();
    let response_blob = build_agent(pem_identity_path)
        .update(&canister_id, "updateBucketCanisterController")
        .with_arg(Encode!(&bucket_canister_id, &contoller).expect("encode piece failed"))
        .call_and_wait(get_waiter())
        .await
        .expect("response error");
    let response = Decode!(&response_blob, bool).unwrap();
    response
}

/// Add a icsp_certified_log admin
///
/// Example code :
/// ```no_run
/// use isp_sdk::isp_certified_log;
///
/// async fn add_admin(icsp_log_canister_id_text: &str, new_admin_text: &str) {
///     isp_certified_log::add_admin(
///         "identities/identity.pem",
///         icsp_log_canister_id_text,
///         new_admin_text,
///     ).await
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let response = add_admin(
///         "4radi-oqaaa-aaaan-qapwa-cai",
///         "bxgws-37y5d-tgmpr-hekbp-y3uxo-yicgs-fo7p3-ccnta-kidrz-74onh-pae",
///     ).await;
/// }
/// ```
pub async fn add_admin(
    pem_identity_path: &str,
    icsp_log_canister_id_text: &str,
    new_admin_text: &str,
) {
    let canister_id = candid::Principal::from_text(icsp_log_canister_id_text).unwrap();
    let new_admin = candid::Principal::from_text(new_admin_text).unwrap();
    let response_blob = build_agent(pem_identity_path)
        .update(&canister_id, "addAdmin")
        .with_arg(Encode!(&new_admin).expect("encode piece failed"))
        .call_and_wait(get_waiter())
        .await
        .expect("response error");
    let response = Decode!(&response_blob, ()).unwrap();
    response
}

/// Delete a icsp_certified_log admin
///
/// Example code :
/// ```no_run
/// use isp_sdk::isp_certified_log;
///
/// async fn delete_admin(icsp_log_canister_id_text: &str, old_admin_text: &str) {
///     isp_certified_log::add_admin(
///         "identities/identity.pem",
///         icsp_log_canister_id_text,
///         old_admin_text,
///     ).await
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let response = delete_admin(
///         "4radi-oqaaa-aaaan-qapwa-cai",
///         "bxgws-37y5d-tgmpr-hekbp-y3uxo-yicgs-fo7p3-ccnta-kidrz-74onh-pae",
///     ).await;
/// }
/// ```
pub async fn delete_admin(
    pem_identity_path: &str,
    icsp_log_canister_id_text: &str,
    old_admin_text: &str,
) {
    let canister_id = candid::Principal::from_text(icsp_log_canister_id_text).unwrap();
    let old_admin = candid::Principal::from_text(old_admin_text).unwrap();
    let response_blob = build_agent(pem_identity_path)
        .update(&canister_id, "deleteAdmin")
        .with_arg(Encode!(&old_admin).expect("encode piece failed"))
        .call_and_wait(get_waiter())
        .await
        .expect("response error");
    let response = Decode!(&response_blob, ()).unwrap();
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
