extern crate isp_sdk;
use candid::{CandidType, Principal};
use isp_sdk::isp_certified_log::{self, Buckets, CertifiedLog, StoreLog};

pub async fn test() {
    let response_1 = get_buckets("4radi-oqaaa-aaaan-qapwa-cai").await;
    match response_1 {
        Some(response) => {
            println!("old buckets:");
            for i in &response.old_buckets {
                println!("{:?}", i.to_text());
            }
            println!("Live Buckets:");
            println!(
                "canister_id:{:?}, used_memory:{:?}",
                response.live_buckets.bucket_id, response.live_buckets.used_memory,
            );
        }
        None => println!("icsp do not have buckets"),
    }

    let response_2 = get_log_num("4radi-oqaaa-aaaan-qapwa-cai").await;
    println!("log num:{:?}", response_2);

    let response_3 = get_logs("4radi-oqaaa-aaaan-qapwa-cai", 0, response_2 - 1).await;
    match response_3 {
        Some(response) => println!("{:?}", response),
        None => println!("no logs"),
    }

    let response_4 = get_admins("4radi-oqaaa-aaaan-qapwa-cai").await;
    println!("admins");
    for i in &response_4 {
        println!("{:?}", i.to_text());
    }

    let response_5 = store(
        "4radi-oqaaa-aaaan-qapwa-cai",
        StoreLog {
            context: "test".to_string(),
        },
    )
    .await;

    let response_6 = add_admin(
        "4radi-oqaaa-aaaan-qapwa-cai",
        "bxgws-37y5d-tgmpr-hekbp-y3uxo-yicgs-fo7p3-ccnta-kidrz-74onh-pae",
    )
    .await;

    let response_7 = delete_admin(
        "4radi-oqaaa-aaaan-qapwa-cai",
        "bxgws-37y5d-tgmpr-hekbp-y3uxo-yicgs-fo7p3-ccnta-kidrz-74onh-pae",
    )
    .await;
}

async fn get_buckets(icsp_log_canister_id_text: &str) -> Option<Buckets> {
    isp_certified_log::get_buckets("identities/identity.pem", icsp_log_canister_id_text).await
}

async fn get_log_num(icsp_log_canister_id_text: &str) -> u128 {
    isp_certified_log::get_log_num("identities/identity.pem", icsp_log_canister_id_text).await
}

async fn get_logs(
    icsp_log_canister_id_text: &str,
    start: u128,
    end: u128,
) -> Option<Vec<CertifiedLog>> {
    isp_certified_log::get_logs(
        "identities/identity.pem",
        icsp_log_canister_id_text,
        start,
        end,
    )
    .await
}

async fn get_admins(icsp_log_canister_id_text: &str) -> Vec<Principal> {
    isp_certified_log::get_admins("identities/identity.pem", icsp_log_canister_id_text).await
}

async fn store(icsp_log_canister_id_text: &str, args: StoreLog) {
    isp_certified_log::store("identities/identity.pem", icsp_log_canister_id_text, args).await
}

async fn add_admin(icsp_log_canister_id_text: &str, new_admin_text: &str) {
    isp_certified_log::add_admin(
        "identities/identity.pem",
        icsp_log_canister_id_text,
        new_admin_text,
    )
    .await
}

async fn delete_admin(icsp_log_canister_id_text: &str, old_admin_text: &str) {
    isp_certified_log::delete_admin(
        "identities/identity.pem",
        icsp_log_canister_id_text,
        old_admin_text,
    )
    .await
}
