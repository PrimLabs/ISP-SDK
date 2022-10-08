extern crate isp_sdk;
use candid::{Nat, Principal};
use icsp::{Buckets, FileBufExt};
use isp::{BurnArgs, BurnResult, CreateICSPResult, TopUpArgs, TopUpResult, TransferResult};
use isp_sdk::{icsp, isp};
use std::fs::OpenOptions;
use std::io::Write;

pub async fn test() {
    let response_1 = get_user_icsps().await;
    for i in &response_1 {
        println!("icsp_name:{:?},icsp_canister_id:{:?}", i.0, i.1.to_text());
    }
    if response_1.is_empty() {
        println!("user do not have icsp\n");
    }

    println!("SubAccount:{:?}\n", get_sub_account().await);

    println!("icp balance:{:?}\n", get_icp_balance().await);

    println!(
        "transfer out icp result:{:?}\n",
        transfer_out_icp(
            "3eee9b4671b8fde5a501288d74d21ee93042dc202104fa35051563ae35d24f2f",
            5000000 as u64,
        )
        .await
    );

    println!("isp admins:");
    for i in &get_isp_admins().await {
        println!("{:?}", Principal::to_text(i));
    }

    let response_4 = create_icsp(
        "icsp-1",
        15_000_000 as u64,
        5_000_000_000_000 as u64 - 2_000_000_000 as u64,
    )
    .await;
    match response_4.0 {
        CreateICSPResult::ok(canister_id) => {
            println!("create icsp success: {:?}", canister_id.to_text());
            println!("use XTC topup result: {:?}", response_4.1.unwrap());
        }
        CreateICSPResult::err(error) => {
            println!("create icsp error: {:?}", error);
        }
    }

    println!(
        "topup icsp result:{:?}\n",
        top_up_icsp(TopUpArgs {
            icsp_canisterId: Principal::from_text("xk2my-yqaaa-aaaal-abdwa-cai").unwrap(),
            icp_amount: 5_000_000 as u64,
        })
        .await
    );

    println!(
        "the file in bucekt:{:?}\n",
        get_bucket_of_file(
            "4radi-oqaaa-aaaan-qapwa-cai",
            "bf0efa3d-6639-4d62-a81d-c90974cc6925",
        )
        .await
        .expect("no bucket have this file")
        .to_text()
    );

    let response_7 = get_icsp_buckets("5ekwd-fyaaa-aaaan-qaxlq-cai").await;
    match response_7 {
        Some(response) => {
            println!("old buckets:");
            for i in &response.old_buckets {
                println!("{:?}", i.to_text());
            }
            println!("Live Buckets:");
            for i in &response.live_buckets {
                println!(
                    "canister_id:{:?}, used_memory:{:?}",
                    i.canister_id.to_text(),
                    i.used_memory,
                );
            }
        }
        None => println!("icsp do not have buckets"),
    }

    println!("icsp admins:");
    for i in &get_icsp_admins("4radi-oqaaa-aaaan-qapwa-cai").await {
        println!("{:?}", i.to_text());
    }

    // url format : icsp_canister_id.raw.ic0.app/'option location'/file_key
    // icsp_canister_id.raw.ic0.app/ic/file_key
    // icsp_canister_id.raw.ic0.app/ipfs/file_key
    // icsp_canister_id.raw.ic0.app/ar/file_key
    for i in &store_files("source/", "4radi-oqaaa-aaaan-qapwa-cai", true).await {
        println!("file_name:{:?},file_key:{:?}", i.0, i.1);
    }

    // url format : icsp_canister_id.raw.ic0.app/'option location'/file_key
    // icsp_canister_id.raw.ic0.app/ic/file_key
    // icsp_canister_id.raw.ic0.app/ipfs/file_key
    // icsp_canister_id.raw.ic0.app/ar/file_key
    let respoonse_8 = store_file("source/bitcoin.pdf", "4radi-oqaaa-aaaan-qapwa-cai", true).await;
    println!("file_name:{:?},file_key:{:?}", respoonse_8.0, respoonse_8.1);

    // url format : icsp_canister_id.raw.ic0.app/'option location'/file_key
    // icsp_canister_id.raw.ic0.app/ic/file_key
    // icsp_canister_id.raw.ic0.app/ipfs/file_key
    // icsp_canister_id.raw.ic0.app/ar/file_key
    println!(
        "store_str, file_key: {:?}",
        store_str(
            "test_isp_sdk_store_str",
            "4radi-oqaaa-aaaan-qapwa-cai",
            true
        )
        .await
    );

    let response_10 = get_file(
        "4radi-oqaaa-aaaan-qapwa-cai",
        "3166112af0dcc940f8e7f2199a4200cfb5e2efb40796391201b8fe9e4ff7ca84",
    )
    .await;
    let mut file = std::fs::File::create("output/bitcoin.pdf").expect("create failed");
    file.write_all(&response_10.0).expect("write failed");
    println!(
        "file out put at folder output/ , file_type:{:?}",
        response_10.1
    );

    let response_12 = add_icsp_admin(
        "4radi-oqaaa-aaaan-qapwa-cai",
        "bxgws-37y5d-tgmpr-hekbp-y3uxo-yicgs-fo7p3-ccnta-kidrz-74onh-pae",
    )
    .await;

    println!(
        "topup icsp with XTC result:{:?}\n",
        top_up_icsp_with_xtc(BurnArgs {
            canister_id: Principal::from_text("hf34l-eyaaa-aaaan-qav5q-cai").unwrap(),
            amount: 1_000_000_000_000 as u64 - 2_000_000_000 as u64,
        })
        .await
    );

    println!(
        "icsp cycle balance:{:?}\n",
        get_cycle_balance("4radi-oqaaa-aaaan-qapwa-cai").await
    );

    println!(
        "get all ic file key result: {:?}",
        get_all_ic_file_key("identities/identity.pem", "4radi-oqaaa-aaaan-qapwa-cai").await
    );

    println!("get file info result:");
    match get_file_info(
        "identities/identity.pem",
        "4radi-oqaaa-aaaan-qapwa-cai",
        "49c1dadd-6fa6-4f15-b963-1a1e6f111028".to_string(),
    )
    .await
    {
        None => println!("do not have this file"),
        Some(file_info) => {
            println!("bucket_id: {:?}", file_info.bucket_id.to_text());
            println!("total_index: {:?}", file_info.total_index);
            println!("received chunk_number: {:?}", file_info.received);
            println!("wrote_page: {:?}", file_info.wrote_page);
            println!("file type: {:?}", file_info.file_type);
            println!("is_http_open: {:?}", file_info.is_http_open);
            println!("total_size: {:?}", file_info.total_size);
        }
    };
}

// return (icsp_name, icsp_canister_id)
async fn get_user_icsps() -> Vec<(String, Principal)> {
    isp::get_user_icsps("identities/identity.pem").await
}

async fn get_sub_account() -> String {
    isp::get_sub_account("identities/identity.pem").await
}

async fn get_isp_admins() -> Vec<Principal> {
    isp::get_isp_admins("identities/identity.pem").await
}

async fn create_icsp(
    icsp_name: &str,
    icp_to_create_amount: u64,
    xtc_to_topup_amount: u64,
) -> (CreateICSPResult, Option<BurnResult>) {
    isp::create_icsp(
        "identities/identity.pem",
        icsp_name,
        icp_to_create_amount,
        xtc_to_topup_amount,
    )
    .await
}

async fn top_up_icsp(args: TopUpArgs) -> TopUpResult {
    isp::top_up_icsp("identities/identity.pem", args).await
}

async fn get_bucket_of_file(icsp_canister_id_text: &str, file_key: &str) -> Option<Principal> {
    icsp::get_bucket_of_file("identities/identity.pem", icsp_canister_id_text, file_key).await
}

async fn get_icsp_buckets(icsp_canister_id_text: &str) -> Option<Buckets> {
    icsp::get_icsp_buckets("identities/identity.pem", icsp_canister_id_text).await
}

async fn get_icsp_admins(icsp_canister_id_text: &str) -> Vec<Principal> {
    icsp::get_icsp_admins("identities/identity.pem", icsp_canister_id_text).await
}

async fn store_files(
    folder_path: &str,
    icsp_canister_id_text: &str,
    is_http_open: bool,
) -> Vec<(String, String)> {
    icsp::store_files(
        "identities/identity.pem",
        folder_path,
        icsp_canister_id_text,
        is_http_open,
    )
    .await
}

async fn store_file(
    file_path_str: &str,
    icsp_canister_id_text: &str,
    is_http_open: bool,
) -> (String, String) {
    icsp::store_file(
        "identities/identity.pem",
        file_path_str,
        icsp_canister_id_text,
        is_http_open,
    )
    .await
}

async fn store_str(data: &str, icsp_canister_id_text: &str, is_http_open: bool) -> String {
    icsp::store_str(
        "identities/identity.pem",
        data,
        icsp_canister_id_text,
        is_http_open,
    )
    .await
}

async fn get_file(icsp_canister_id_text: &str, file_key: &str) -> (Vec<u8>, String) {
    icsp::get_file("identities/identity.pem", icsp_canister_id_text, file_key).await
}

async fn add_icsp_admin(icsp_canister_id_text: &str, new_admin_text: &str) {
    icsp::add_icsp_admin(
        "identities/identity.pem",
        icsp_canister_id_text,
        new_admin_text,
    )
    .await
}

async fn delete_icsp_admin(icsp_canister_id_text: &str, old_admin_text: &str) {
    icsp::delete_icsp_admin(
        "identities/identity.pem",
        icsp_canister_id_text,
        old_admin_text,
    )
    .await
}

async fn top_up_icsp_with_xtc(args: BurnArgs) -> BurnResult {
    isp::top_up_icsp_with_xtc("identities/identity.pem", args).await
}

async fn get_icp_balance() -> u64 {
    isp::get_icp_balance("identities/identity.pem").await
}

async fn transfer_out_icp(to: &str, amount: u64) -> TransferResult {
    isp::transfer_out_icp("identities/identity.pem", to, amount).await
}

async fn get_cycle_balance(icsp_canister_id_text: &str) -> Nat {
    icsp::get_cycle_balance("identities/identity.pem", icsp_canister_id_text).await
}

async fn get_all_ic_file_key(pem_identity_path: &str, icsp_canister_id_text: &str) -> Vec<String> {
    icsp::get_all_ic_file_key("identities/identity.pem", icsp_canister_id_text).await
}

async fn get_file_info(
    pem_identity_path: &str,
    icsp_canister_id_text: &str,
    file_key: String,
) -> Option<FileBufExt> {
    icsp::get_file_info("identities/identity.pem", icsp_canister_id_text, file_key).await
}
