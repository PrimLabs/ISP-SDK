extern crate isp_sdk;
use candid::Principal;
use isp::{BurnArgs, CreateICSPResult, TopUpArgs};
use isp_sdk::{icsp, isp};
use std::io::Write;

pub async fn get_user_icsps() {
    let response = isp::get_user_icsps("identities/identity.pem").await;
    for i in &response {
        println!("icsp_name:{:?},icsp_canister_id:{:?}", i.0, i.1.to_text());
    }
    if response.is_empty() {
        println!("user do not have icsp\n");
    }
}

pub async fn get_sub_account() {
    println!(
        "SubAccount:{:?}\n",
        isp::get_sub_account("identities/identity.pem").await
    );
}

pub async fn get_isp_admins() {
    println!("isp admins:");
    for i in &isp::get_isp_admins("identities/identity.pem").await {
        println!("{:?}", Principal::to_text(i));
    }
}

pub async fn get_isp_version() {
    println!(
        "isp version: {:?}",
        isp::get_version("identities/identity.pem").await
    );
}

pub async fn get_icsp_version() {
    println!(
        "icsp version: {:?}",
        icsp::get_version("identities/identity.pem", "4radi-oqaaa-aaaan-qapwa-cai").await
    );
}

pub async fn get_ic_file_numbers() {
    println!(
        "icsp 's ic file numbers: {:?}",
        icsp::get_ic_file_numbers("identities/identity.pem", "4radi-oqaaa-aaaan-qapwa-cai")
            .await
            .unwrap()
    );
}

pub async fn get_field_file_infos() {
    let page_num: u64 = 10;
    let page_index: u64 = 2;
    println!(
        "every page have {:?} file_info, query the {:?} page\n",
        page_num, page_index
    );
    let mut index = 0;
    for file_info in &icsp::get_field_file_infos(
        "identities/identity.pem",
        "4radi-oqaaa-aaaan-qapwa-cai",
        page_num,
        page_index,
    )
    .await
    {
        index += 1;
        println!("the file_info index: {:?}", index);
        println!("bucket_id: {:?}", file_info.bucket_id.to_text());
        println!("total_index: {:?}", file_info.total_index);
        println!("received chunk_number: {:?}", file_info.received);
        println!("wrote_page: {:?}", file_info.wrote_page);
        println!("file type: {:?}", file_info.file_type);
        println!("is_http_open: {:?}", file_info.is_http_open);
        println!("total_size: {:?}", file_info.total_size);
        println!("\n");
    }
}

pub async fn create_icsp() {
    let response = isp::create_icsp(
        "identities/identity.pem",
        "icsp-1",
        15_000_000 as u64,
        5_000_000_000_000 as u64 - 2_000_000_000 as u64,
    )
    .await;
    match response.0 {
        CreateICSPResult::ok(canister_id) => {
            println!("create icsp success: {:?}", canister_id.to_text());
            println!("use XTC topup result: {:?}", response.1.unwrap());
        }
        CreateICSPResult::err(error) => {
            println!("create icsp error: {:?}", error);
        }
    }
}

pub async fn top_up_icsp() {
    println!(
        "topup icsp result:{:?}\n",
        isp::top_up_icsp(
            "identities/identity.pem",
            TopUpArgs {
                icsp_canisterId: Principal::from_text("xk2my-yqaaa-aaaal-abdwa-cai").unwrap(),
                icp_amount: 5_000_000 as u64,
            }
        )
        .await
    );
}

pub async fn get_bucket_of_file() {
    println!(
        "the file in bucekt:{:?}\n",
        icsp::get_bucket_of_file(
            "identities/identity.pem",
            "4radi-oqaaa-aaaan-qapwa-cai",
            "c3fc027b-0917-4308-adf5-bdd460598d88",
        )
        .await
        .expect("no bucket have this file")
        .to_text()
    );
}

pub async fn get_icsp_buckets() {
    let response =
        icsp::get_icsp_buckets("identities/identity.pem", "4radi-oqaaa-aaaan-qapwa-cai").await;
    match response {
        Some(response) => {
            println!("dead buckets:");
            for i in &response.dead_buckets {
                println!(
                    "canister_id: {:?}; used_memory: {:?}",
                    i.canister_id.to_text(),
                    i.used_memory
                );
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
}

pub async fn get_icsp_admins() {
    println!("icsp admins:");
    for i in &icsp::get_icsp_admins("identities/identity.pem", "4radi-oqaaa-aaaan-qapwa-cai").await
    {
        println!("{:?}", i.to_text());
    }
}

pub async fn top_up_bucket() {
    // 0.1 T Cycles
    icsp::top_up_bucket(
        "identities/identity.pem",
        "4radi-oqaaa-aaaan-qapwa-cai",
        100_000_000_000 as u64,
    )
    .await;
    println!("complete top_up_bucket func, top up every bucket 0.1 T Cycles");
}

pub async fn store_files() {
    // url format : icsp_canister_id.raw.ic0.app/'option location'/file_key
    // icsp_canister_id.raw.ic0.app/ic/file_key
    // icsp_canister_id.raw.ic0.app/ipfs/file_key
    // icsp_canister_id.raw.ic0.app/ar/file_key
    for i in &icsp::store_files(
        "identities/identity.pem",
        "/Users/heyuanxun/Downloads/90MSource/",
        "4radi-oqaaa-aaaan-qapwa-cai",
        true,
    )
    .await
    {
        println!("file_name:{:?},file_key:{:?}", i.0, i.1);
    }
}

pub async fn store_file() {
    // url format : icsp_canister_id.raw.ic0.app/'option location'/file_key
    // icsp_canister_id.raw.ic0.app/ic/file_key
    // icsp_canister_id.raw.ic0.app/ipfs/file_key
    // icsp_canister_id.raw.ic0.app/ar/file_key
    let respoonse = icsp::store_file(
        "identities/identity.pem",
        "source/a.jpeg",
        "4radi-oqaaa-aaaan-qapwa-cai",
        true,
    )
    .await;
    println!("file_name:{:?},file_key:{:?}", respoonse.0, respoonse.1);
}

pub async fn store_file_by_key() {
    let respoonse = icsp::store_file_by_key(
        "identities/identity.pem",
        "source/bitcoin.pdf",
        "4radi-oqaaa-aaaan-qapwa-cai",
        true,
        "test_key".to_string(),
    )
    .await;
    println!("file_name:{:?},file_key:{:?}", respoonse.0, respoonse.1);
}

pub async fn delete_file() {
    let _respoonse = icsp::delete_file(
        "identities/identity.pem",
        "4radi-oqaaa-aaaan-qapwa-cai",
        "7d207a64-8621-419b-a4be-022591f4fd6e",
    )
    .await;
    println!("complete delete file func");
}

pub async fn store_str() {
    // url format : icsp_canister_id.raw.ic0.app/'option location'/file_key
    // icsp_canister_id.raw.ic0.app/ic/file_key
    // icsp_canister_id.raw.ic0.app/ipfs/file_key
    // icsp_canister_id.raw.ic0.app/ar/file_key
    println!(
        "store_str, file_key: {:?}",
        icsp::store_str(
            "identities/identity.pem",
            "4radi-oqaaa-aaaan-qapwa-cai",
            "test_isp_sdk_store_str",
            true,
        )
        .await
    );
}

pub async fn replace_str() {
    icsp::replace_str(
        "identities/identity.pem",
        "4radi-oqaaa-aaaan-qapwa-cai",
        "8225a448-7eff-4162-bb52-313884bbde4e",
        "test_isp_sdk_replace_str",
        true,
    )
    .await;
    println!("replace_str complete ");
}

pub async fn get_file() {
    let response = icsp::get_file(
        "identities/identity.pem",
        "4radi-oqaaa-aaaan-qapwa-cai",
        "3166112af0dcc940f8e7f2199a4200cfb5e2efb40796391201b8fe9e4ff7ca84",
    )
    .await;

    let mut file = std::fs::File::create("output/bitcoin.pdf").expect("create failed");
    file.write_all(&response.0).expect("write failed");

    println!(
        "file out put at folder output/ , file_type:{:?}",
        response.1
    );
}

pub async fn add_icsp_admin() {
    icsp::add_icsp_admin(
        "identities/identity.pem",
        "4radi-oqaaa-aaaan-qapwa-cai",
        "bxgws-37y5d-tgmpr-hekbp-y3uxo-yicgs-fo7p3-ccnta-kidrz-74onh-pae",
    )
    .await
}

pub async fn delete_icsp_admin() {
    icsp::delete_icsp_admin(
        "identities/identity.pem",
        "4radi-oqaaa-aaaan-qapwa-cai",
        "bxgws-37y5d-tgmpr-hekbp-y3uxo-yicgs-fo7p3-ccnta-kidrz-74onh-pae",
    )
    .await
}

pub async fn top_up_icsp_with_xtc() {
    println!(
        "topup icsp with XTC result:{:?}\n",
        isp::top_up_icsp_with_xtc(
            "identities/identity.pem",
            BurnArgs {
                canister_id: Principal::from_text("hf34l-eyaaa-aaaan-qav5q-cai").unwrap(),
                amount: 1_000_000_000_000 as u64 - 2_000_000_000 as u64,
            }
        )
        .await
    );
}

pub async fn get_user_sub_account_icp_balance() {
    println!(
        "icp balance:{:?}\n",
        isp::get_user_sub_account_icp_balance("identities/identity.pem").await
    );
}

pub async fn transfer_out_user_sub_account_icp() {
    println!(
        "transfer out icp result:{:?}\n",
        isp::transfer_out_user_sub_account_icp(
            "identities/identity.pem",
            "3eee9b4671b8fde5a501288d74d21ee93042dc202104fa35051563ae35d24f2f",
            5000000 as u64
        )
        .await
    );
}

pub async fn get_cycle_balance() {
    println!(
        "icsp cycle balance:{:?}\n",
        icsp::get_cycle_balance("identities/identity.pem", "4radi-oqaaa-aaaan-qapwa-cai").await
    );
}

pub async fn get_all_ic_file_key() {
    println!(
        "get all ic file key result: {:?}",
        icsp::get_all_ic_file_key("identities/identity.pem", "4radi-oqaaa-aaaan-qapwa-cai").await
    );
}

pub async fn get_file_info() {
    println!("get file info result:");
    match icsp::get_file_info(
        "identities/identity.pem",
        "4radi-oqaaa-aaaan-qapwa-cai",
        "c3fc027b-0917-4308-adf5-bdd460598d88".to_string(),
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
