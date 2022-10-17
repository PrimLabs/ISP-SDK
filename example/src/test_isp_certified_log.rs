extern crate isp_sdk;
use isp_sdk::isp_certified_log::{self, StoreLog};

pub async fn get_buckets() {
    let response =
        isp_certified_log::get_buckets("identities/identity.pem", "4radi-oqaaa-aaaan-qapwa-cai")
            .await;
    match response {
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
}

pub async fn get_log_num() {
    println!(
        "log num:{:?}",
        isp_certified_log::get_log_num("identities/identity.pem", "4radi-oqaaa-aaaan-qapwa-cai")
            .await
    );
}

pub async fn get_logs() {
    let response = isp_certified_log::get_logs(
        "identities/identity.pem",
        "4radi-oqaaa-aaaan-qapwa-cai",
        0,
        isp_certified_log::get_log_num("identities/identity.pem", "4radi-oqaaa-aaaan-qapwa-cai")
            .await
            - 1,
    )
    .await;
    match response {
        Some(response) => println!("{:?}", response),
        None => println!("no logs"),
    }
}

pub async fn get_admins() {
    println!("admins");
    for i in
        &isp_certified_log::get_admins("identities/identity.pem", "4radi-oqaaa-aaaan-qapwa-cai")
            .await
    {
        println!("{:?}", i.to_text());
    }
}

pub async fn store() {
    isp_certified_log::store(
        "identities/identity.pem",
        "4radi-oqaaa-aaaan-qapwa-cai",
        StoreLog {
            context: "test".to_string(),
        },
    )
    .await;
}

pub async fn add_admin() {
    isp_certified_log::add_admin(
        "identities/identity.pem",
        "4radi-oqaaa-aaaan-qapwa-cai",
        "bxgws-37y5d-tgmpr-hekbp-y3uxo-yicgs-fo7p3-ccnta-kidrz-74onh-pae",
    )
    .await;
}

pub async fn delete_admin() {
    isp_certified_log::delete_admin(
        "identities/identity.pem",
        "4radi-oqaaa-aaaan-qapwa-cai",
        "bxgws-37y5d-tgmpr-hekbp-y3uxo-yicgs-fo7p3-ccnta-kidrz-74onh-pae",
    )
    .await;
}
