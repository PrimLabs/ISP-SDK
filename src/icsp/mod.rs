use candid::{Decode, Encode, Nat};
use garcon::Delay;
use ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport;
use ic_agent::{identity::Secp256k1Identity, Agent};
use std::fs::{self};
use std::path::Path;
use uuid::Uuid;
mod icsp_did;
pub use icsp_did::{Buckets, FileBufExt, StoreArgs};

const UPDATE_SIZE: usize = 2031616;

/// Get all ic files 's key from user 's icsp
///
/// # Examples
///
/// ``` no_run
/// use isp_sdk::icsp;
///
/// pub async fn get_all_ic_file_key() {
///   println!(
///     "get all ic file key result: {:?}",
///     icsp::get_all_ic_file_key("identities/identity.pem", "4radi-oqaaa-aaaan-qapwa-cai").await
///  );
/// }
/// ```
pub async fn get_all_ic_file_key(
    pem_identity_path: &str,
    icsp_canister_id_text: &str,
) -> Vec<String> {
    let canister_id = candid::Principal::from_text(icsp_canister_id_text).unwrap();
    let response_blob = build_agent(pem_identity_path)
        .query(&canister_id, "getAllIcFileKey")
        .with_arg(Encode!().expect("encode piece failed"))
        .call()
        .await
        .expect("response error");
    Decode!(&response_blob, Vec<String>).unwrap()
}

/// Get file's information
///
/// # Examples
///
/// ``` no_run
/// use isp_sdk::icsp;
/// pub async fn get_file_info() {
///    println!("get file info result:");
///    match icsp::get_file_info(
///        "identities/identity.pem",
///        "4radi-oqaaa-aaaan-qapwa-cai",
///        "49c1dadd-6fa6-4f15-b963-1a1e6f111028".to_string(),
///    )
///        .await
///    {
///        None => println!("do not have this file"),
///        Some(file_info) => {
///            println!("bucket_id: {:?}", file_info.bucket_id.to_text());
///            println!("total_index: {:?}", file_info.total_index);
///            println!("received chunk_number: {:?}", file_info.received);
///            println!("wrote_page: {:?}", file_info.wrote_page);
///            println!("file type: {:?}", file_info.file_type);
///            println!("is_http_open: {:?}", file_info.is_http_open);
///            println!("total_size: {:?}", file_info.total_size);
///        }
///    };
/// }
/// ```
pub async fn get_file_info(
    pem_identity_path: &str,
    icsp_canister_id_text: &str,
    file_key: String,
) -> Option<FileBufExt> {
    let canister_id = candid::Principal::from_text(icsp_canister_id_text).unwrap();
    let response_blob = build_agent(pem_identity_path)
        .query(&canister_id, "getFileInfo")
        .with_arg(Encode!(&file_key).expect("encode piece failed"))
        .call()
        .await
        .expect("response error");
    Decode!(&response_blob, Option<FileBufExt>).unwrap()
}

/// Get icsp 's cycle balance
///
/// The cycle balance is e12s
///
/// # Examples
///
/// ``` no_run
/// use isp_sdk::icsp;
///
/// pub async fn get_cycle_balance() {
///     println!(
///         "icsp cycle balance:{:?}\n",
///         icsp::get_cycle_balance("identities/identity.pem", "4radi-oqaaa-aaaan-qapwa-cai").await
///     );
/// }
/// ```
pub async fn get_cycle_balance(pem_identity_path: &str, icsp_canister_id_text: &str) -> Nat {
    let canister_id = candid::Principal::from_text(icsp_canister_id_text).unwrap();
    let response_blob = build_agent(pem_identity_path)
        .query(&canister_id, "getCycleBalance")
        .with_arg(Encode!().expect("encode piece failed"))
        .call()
        .await
        .expect("response error");
    Decode!(&response_blob, Nat).unwrap()
}

/// Get the bucket where the file is stored
///
/// # Examples
///
/// ``` no_run
/// use isp_sdk::icsp;
///
/// pub async fn get_bucket_of_file() {
///     println!(
///         "the file in bucekt:{:?}\n",
///         icsp::get_bucket_of_file(
///             "identities/identity.pem",
///             "4radi-oqaaa-aaaan-qapwa-cai",
///             "bf0efa3d-6639-4d62-a81d-c90974cc6925",
///         )
///             .await
///             .expect("no bucket have this file")
///             .to_text()
///     );
/// }
/// ```
pub async fn get_bucket_of_file(
    pem_identity_path: &str,
    icsp_canister_id_text: &str,
    file_key: &str,
) -> Option<candid::Principal> {
    let canister_id = candid::Principal::from_text(icsp_canister_id_text).unwrap();
    let response_blob = build_agent(pem_identity_path)
        .query(&canister_id, "getBucketOfFile")
        .with_arg(Encode!(&file_key).expect("encode piece failed"))
        .call()
        .await
        .expect("response error");
    Decode!(&response_blob, Option<candid::Principal>).unwrap()
}

/// Get buckets of user's icsp
///
/// # Examples
///
/// ```no_run
/// use isp_sdk::icsp;
///
/// pub async fn get_icsp_buckets() {
///     let response =
///         icsp::get_icsp_buckets("identities/identity.pem", "5ekwd-fyaaa-aaaan-qaxlq-cai").await;
///     match response {
///         Some(response) => {
///             println!("old buckets:");
///             for i in &response.old_buckets {
///                 println!("{:?}", i.to_text());
///             }
///             println!("Live Buckets:");
///             for i in &response.live_buckets {
///                 println!(
///                     "canister_id:{:?}, used_memory:{:?}",
///                     i.canister_id.to_text(),
///                     i.used_memory,
///                 );
///             }
///         }
///         None => println!("icsp do not have buckets"),
///     }
/// }
/// ```
pub async fn get_icsp_buckets(
    pem_identity_path: &str,
    icsp_canister_id_text: &str,
) -> Option<Buckets> {
    let canister_id = candid::Principal::from_text(icsp_canister_id_text).unwrap();
    let response_blob = build_agent(pem_identity_path)
        .query(&canister_id, "getBuckets")
        .with_arg(Encode!().expect("encode piece failed"))
        .call()
        .await
        .expect("response error");
    Decode!(&response_blob, Option<Buckets>).unwrap()
}

/// Get icsp 's admins
///
/// # Examples
///
/// ``` no_run
/// use isp_sdk::icsp;
/// pub async fn get_icsp_admins() {
///     println!("icsp admins:");
///     for i in &icsp::get_icsp_admins("identities/identity.pem", "4radi-oqaaa-aaaan-qapwa-cai").await
///     {
///         println!("{:?}", i.to_text());
///     }
/// }
/// ```
pub async fn get_icsp_admins(
    pem_identity_path: &str,
    icsp_canister_id_text: &str,
) -> Vec<candid::Principal> {
    let canister_id = candid::Principal::from_text(icsp_canister_id_text).unwrap();
    let response_blob = build_agent(pem_identity_path)
        .query(&canister_id, "getAdmins")
        .with_arg(Encode!().expect("encode error"))
        .call()
        .await
        .expect("response error");
    Decode!(&response_blob, Vec<candid::Principal>).unwrap()
}

/// Store files from folder_path
///
/// If http open,url format: icsp_canister_id.raw.ic0.app/ic/file_key
///
/// # Examples
///
/// ``` no_run
/// use isp_sdk::icsp;
///
/// pub async fn store_files() {
///     // url format : icsp_canister_id.raw.ic0.app/'option location'/file_key
///     // icsp_canister_id.raw.ic0.app/ic/file_key
///     // icsp_canister_id.raw.ic0.app/ipfs/file_key
///     // icsp_canister_id.raw.ic0.app/ar/file_key
///     for i in &icsp::store_files(
///         "identities/identity.pem",
///         "source/",
///         "4radi-oqaaa-aaaan-qapwa-cai",
///         true,
///     )
///         .await
///     {
///         println!("file_name:{:?},file_key:{:?}", i.0, i.1);
///     }
/// }
/// ```
pub async fn store_files(
    pem_identity_path: &str,
    folder_path: &str,
    icsp_canister_id_text: &str,
    is_http_open: bool,
) -> Vec<(String, String)> {
    let canister_id = candid::Principal::from_text(icsp_canister_id_text).unwrap();
    let agent = build_agent(pem_identity_path);

    let mut ans: Vec<(String, String)> = Vec::new();
    let paths = fs::read_dir(&folder_path).unwrap();
    for path in paths {
        let file_path = path.unwrap().file_name().into_string().unwrap();
        let pos: Vec<&str> = file_path.split(".").collect();
        let file_name = String::from(pos[0]);
        let file_type = String::from(pos[1]);
        let file_extension = String::from(get_file_type(&file_type));
        let s = folder_path.to_owned() + &file_path;

        let (file_size, data_slice) = get_file_from_source(&s);
        let file_key = Uuid::new_v4().to_string();
        let puts = build_store_args(
            file_key.clone(),
            file_extension,
            file_size.try_into().unwrap(),
            &data_slice,
            is_http_open,
        );
        for put in &puts {
            let _response_blob = agent
                .update(&canister_id, "store")
                .with_arg(Encode!(put).expect("encode piece failed"))
                .call_and_wait(get_waiter())
                .await
                .expect("response error");
        }
        ans.push((file_name.clone(), file_key.clone()));
    }
    ans
}

/// Store a file from file_path
///
/// return (file_name, file_key)
///
/// If http open,url format: icsp_canister_id.raw.ic0.app/ic/file_key
///
/// # Examples
///
/// ``` no_run
/// use isp_sdk::icsp;
///
/// pub async fn store_file() {
///     // url format : icsp_canister_id.raw.ic0.app/'option location'/file_key
///     // icsp_canister_id.raw.ic0.app/ic/file_key
///     // icsp_canister_id.raw.ic0.app/ipfs/file_key
///     // icsp_canister_id.raw.ic0.app/ar/file_key
///     let respoonse = icsp::store_file(
///         "identities/identity.pem",
///         "source/bitcoin.pdf",
///         "4radi-oqaaa-aaaan-qapwa-cai",
///         true,
///     )
///         .await;
///     println!("file_name:{:?},file_key:{:?}", respoonse.0, respoonse.1);
/// }
/// ```
pub async fn store_file(
    pem_identity_path: &str,
    file_path_str: &str,
    icsp_canister_id_text: &str,
    is_http_open: bool,
) -> (String, String) {
    let canister_id = candid::Principal::from_text(icsp_canister_id_text).unwrap();
    let agent = build_agent(pem_identity_path);
    let file_path = Path::new(file_path_str);
    let file_name = file_path.file_stem().unwrap().to_str().unwrap().to_owned();
    let file_extension = String::from(get_file_type(
        file_path.extension().unwrap().to_str().unwrap(),
    ));

    let (file_size, data_slice) = get_file_from_source(file_path_str);
    let file_key = Uuid::new_v4().to_string();
    let puts = build_store_args(
        file_key.clone(),
        file_extension,
        file_size.try_into().unwrap(),
        &data_slice,
        is_http_open,
    );
    for put in &puts {
        let _response_blob = agent
            .update(&canister_id, "store")
            .with_arg(Encode!(put).expect("encode piece failed"))
            .call_and_wait(get_waiter())
            .await
            .expect("response error");
    }
    (file_name, file_key.clone())
}

/// Store file with given key
///
/// return (file_name, file_key)
///
/// If http open,url format: icsp_canister_id.raw.ic0.app/ic/file_key
///
/// # Examples
///
/// ``` no_run
/// use isp_sdk::icsp;
///
/// pub async fn store_file_by_key() {
///     let respoonse = icsp::store_file_by_key(
///         "identities/identity.pem",
///         "source/bitcoin.pdf",
///         "4radi-oqaaa-aaaan-qapwa-cai",
///         true,
///         "test_key".to_string(),
///     )
///         .await;
///     println!("file_name:{:?},file_key:{:?}", respoonse.0, respoonse.1);
/// }
/// ```
pub async fn store_file_by_key(
    pem_identity_path: &str,
    file_path_str: &str,
    icsp_canister_id_text: &str,
    is_http_open: bool,
    file_key: String,
) -> (String, String) {
    let canister_id = candid::Principal::from_text(icsp_canister_id_text).unwrap();
    let agent = build_agent(pem_identity_path);
    let file_path = Path::new(file_path_str);
    let file_name = file_path.file_stem().unwrap().to_str().unwrap().to_owned();
    let file_extension = String::from(get_file_type(
        file_path.extension().unwrap().to_str().unwrap(),
    ));

    let (file_size, data_slice) = get_file_from_source(file_path_str);
    let puts = build_store_args(
        file_key.clone(),
        file_extension,
        file_size.try_into().unwrap(),
        &data_slice,
        is_http_open,
    );
    for put in &puts {
        let _response_blob = agent
            .update(&canister_id, "store")
            .with_arg(Encode!(put).expect("encode piece failed"))
            .call_and_wait(get_waiter())
            .await
            .expect("response error");
    }
    (file_name, file_key.clone())
}

/// Delete file by file_key
///
/// # Examples
///
/// ``` no_run
/// use isp_sdk::icsp;
///
/// pub async fn delete_file() {
///     let _respoonse = icsp::delete_file(
///         "identities/identity.pem",
///         "5ekwd-fyaaa-aaaan-qaxlq-cai",
///         "64b9eb91-feaa-43f0-aa39-3040c035c5bb",
///     )
///         .await;
///     println!("complete delete file func");
/// }
/// ```
pub async fn delete_file(pem_identity_path: &str, icsp_canister_id_text: &str, file_key: &str) {
    let canister_id = candid::Principal::from_text(icsp_canister_id_text).unwrap();
    let agent = build_agent(pem_identity_path);
    let _ = agent
        .update(&canister_id, "delete")
        .with_arg(Encode!(&file_key.to_string()).expect("encode piece failed"))
        .call_and_wait(get_waiter())
        .await
        .expect("response error");
}

/// Store str data
///
/// If http open,url format: icsp_canister_id.raw.ic0.app/ic/file_key
///
/// # Examples
///
/// ``` no_run
/// use isp_sdk::icsp;
///
/// pub async fn store_str() {
///     // url format : icsp_canister_id.raw.ic0.app/'option location'/file_key
///     // icsp_canister_id.raw.ic0.app/ic/file_key
///     // icsp_canister_id.raw.ic0.app/ipfs/file_key
///     // icsp_canister_id.raw.ic0.app/ar/file_key
///     println!(
///         "store_str, file_key: {:?}",
///         icsp::store_str(
///             "identities/identity.pem",
///             "4radi-oqaaa-aaaan-qapwa-cai",
///             "test_isp_sdk_store_str",
///             true,
///         )
///             .await
///     );
/// }
/// ```
pub async fn store_str(
    pem_identity_path: &str,
    icsp_canister_id_text: &str,
    data: &str,
    is_http_open: bool,
) -> String {
    let canister_id = candid::Principal::from_text(icsp_canister_id_text).unwrap();
    let file_extension = "text/plain".to_string();
    let file_size = data.len();
    let file_key = Uuid::new_v4().to_string();

    let put = StoreArgs {
        key: file_key.clone(),
        value: data.as_bytes().to_owned(),
        total_index: Nat::from(1),
        file_type: file_extension.clone(),
        total_size: file_size.clone() as u64,
        is_http_open: is_http_open.clone(),
        index: Nat::from(0),
    };
    let _ = build_agent(pem_identity_path)
        .update(&canister_id, "store")
        .with_arg(Encode!(&put).expect("encode piece failed"))
        .call_and_wait(get_waiter())
        .await
        .expect("response error");

    file_key
}

/// Replace the value str corresponding to the key
/// # Examples
///
/// ``` no_run
/// use isp_sdk::icsp;
///
/// pub async fn replace_str() {
///     icsp::replace_str(
///         "identities/identity.pem",
///         "4radi-oqaaa-aaaan-qapwa-cai",
///         "8225a448-7eff-4162-bb52-313884bbde4e",
///         "test_isp_sdk_replace_str",
///         true,
///     )
///         .await;
///     println!("replace_str complete ");
/// }
/// ```
pub async fn replace_str(
    pem_identity_path: &str,
    icsp_canister_id_text: &str,
    file_key: &str,
    data: &str,
    is_http_open: bool,
) {
    delete_file(pem_identity_path, icsp_canister_id_text, file_key).await;

    let canister_id = candid::Principal::from_text(icsp_canister_id_text).unwrap();
    let put = StoreArgs {
        key: file_key.to_string().to_owned(),
        value: data.as_bytes().to_owned(),
        total_index: Nat::from(1),
        file_type: "text/plain".to_string().clone(),
        total_size: data.len().clone() as u64,
        is_http_open: is_http_open.clone(),
        index: Nat::from(0),
    };
    let _ = build_agent(pem_identity_path)
        .update(&canister_id, "store")
        .with_arg(Encode!(&put).expect("encode piece failed"))
        .call_and_wait(get_waiter())
        .await
        .expect("response error");
}

/// Get file from icsp, return (data, file_type)
///
/// # Examples
///
/// ``` no_run
/// use isp_sdk::icsp;
/// use std::io::Write;
/// pub async fn get_file() {
///     let response = icsp::get_file(
///         "identities/identity.pem",
///         "4radi-oqaaa-aaaan-qapwa-cai",
///         "3166112af0dcc940f8e7f2199a4200cfb5e2efb40796391201b8fe9e4ff7ca84",
///     )
///         .await;
///
///     let mut file = std::fs::File::create("output/bitcoin.pdf").expect("create failed");
///     file.write_all(&response.0).expect("write failed");
///
///     println!(
///         "file out put at folder output/ , file_type:{:?}",
///         response.1
///     );
/// }
/// ```
pub async fn get_file(
    pem_identity_path: &str,
    icsp_canister_id_text: &str,
    file_key: &str,
) -> (Vec<u8>, String) {
    let bucket_canister_id = get_bucket_of_file(pem_identity_path, icsp_canister_id_text, file_key)
        .await
        .expect("can not find bucket have this file");
    let agent = build_agent(pem_identity_path);

    let total_index_blob = agent
        .update(
            &candid::Principal::from_text(bucket_canister_id.to_text()).unwrap(),
            "getFileTotalIndex",
        )
        .with_arg(Encode!(&file_key).expect("encode failed"))
        .call_and_wait(get_waiter())
        .await
        .expect("response error");
    let total_index = Decode!(&total_index_blob, Nat).unwrap();
    if total_index < Nat::from(1) {
        return (vec![], "".to_string());
    }
    let mut index = 0;
    let mut payload: Vec<u8> = Vec::new();
    let mut file_type = "".to_string();
    while Nat::from(index) < total_index {
        let response_blob = agent
            .query(
                &candid::Principal::from_text(bucket_canister_id.to_text()).unwrap(),
                "get",
            )
            .with_arg(Encode!(&file_key, &Nat::from(index)).expect("encode failed"))
            .call()
            .await
            .expect("response error");
        let mut response = Decode!(&response_blob, Option<(Vec<u8>, String)>)
            .unwrap()
            .expect("assets not have this file");
        payload.append(&mut response.0);
        index += 1;
        if Nat::from(index) == total_index {
            file_type = response.1;
        }
    }

    (payload, file_type)
}

/// Add admin of icsp
///
/// # Examples
///
/// ``` no_run
/// use isp_sdk::icsp;
///
/// pub async fn add_icsp_admin() {
///     icsp::add_icsp_admin(
///         "identities/identity.pem",
///         "4radi-oqaaa-aaaan-qapwa-cai",
///         "bxgws-37y5d-tgmpr-hekbp-y3uxo-yicgs-fo7p3-ccnta-kidrz-74onh-pae",
///     )
///         .await
/// }
/// ```
pub async fn add_icsp_admin(
    pem_identity_path: &str,
    icsp_canister_id_text: &str,
    new_admin_text: &str,
) {
    let canister_id = candid::Principal::from_text(icsp_canister_id_text).unwrap();
    let new_admin = candid::Principal::from_text(new_admin_text).unwrap();
    let _ = build_agent(pem_identity_path)
        .update(&canister_id, "addAdmin")
        .with_arg(Encode!(&new_admin).expect("encode error"))
        .call_and_wait(get_waiter())
        .await
        .expect("response error");
}

/// Delete admin of icsp
///
/// # Examples
///
/// ``` no_run
/// use isp_sdk::icsp;
///
/// pub async fn delete_icsp_admin() {
///     icsp::delete_icsp_admin(
///         "identities/identity.pem",
///         "4radi-oqaaa-aaaan-qapwa-cai",
///         "bxgws-37y5d-tgmpr-hekbp-y3uxo-yicgs-fo7p3-ccnta-kidrz-74onh-pae",
///     )
///         .await
/// }
/// ```
pub async fn delete_icsp_admin(
    pem_identity_path: &str,
    icsp_canister_id_text: &str,
    old_admin_text: &str,
) {
    let canister_id = candid::Principal::from_text(icsp_canister_id_text).unwrap();
    let old_admin = candid::Principal::from_text(old_admin_text).unwrap();
    let _ = build_agent(pem_identity_path)
        .update(&canister_id, "deleteAdmin")
        .with_arg(Encode!(&old_admin).expect("encode error"))
        .call_and_wait(get_waiter())
        .await
        .expect("response error");
}

/// Top up every bucket some Cycles by using icsp's Cycles
///
/// # Examples
///
/// ``` no_run
/// use isp_sdk::icsp;
///
/// pub async fn top_up_bucket() {
///     // 0.1 T Cycles
///     icsp::top_up_bucket(
///         "identities/identity.pem",
///         "4radi-oqaaa-aaaan-qapwa-cai",
///         100_000_000_000 as u64,
///     )
///         .await;
///     println!("complete top_up_bucket func, top up every bucket 0.1 T Cycles");
/// }
/// ```
pub async fn top_up_bucket(pem_identity_path: &str, icsp_canister_id_text: &str, amount: u64) {
    let canister_id = candid::Principal::from_text(icsp_canister_id_text).unwrap();
    let _ = build_agent(pem_identity_path)
        .update(&canister_id, "topUpBucket")
        .with_arg(Encode!(&Nat::from(amount)).expect("encode error"))
        .call_and_wait(get_waiter())
        .await
        .expect("response error");
}

/// Get ICSP's WASM version
///
/// # Examples
///
/// ``` no_run
/// use isp_sdk::icsp;
///
/// pub async fn get_icsp_version() {
///     println!(
///         "icsp version: {:?}",
///         icsp::get_version("identities/identity.pem", "4radi-oqaaa-aaaan-qapwa-cai").await
///     );
/// }
/// ```
pub async fn get_version(pem_identity_path: &str, icsp_canister_id_text: &str) -> String {
    let canister_id = candid::Principal::from_text(icsp_canister_id_text).unwrap();
    let response_blob = build_agent(pem_identity_path)
        .query(&canister_id, "getVersion")
        .with_arg(Encode!().expect("encode error"))
        .call()
        .await
        .expect("response error");
    Decode!(&response_blob, String).unwrap()
}

/// Query the number of ic files stored in icsp
///
/// # Examples
///
/// ``` no_run
/// use isp_sdk::icsp;
///
/// pub async fn get_ic_file_numbers(pem_identity_path: &str, icsp_canister_id_text: &str) -> Nat {
///     println!(
///         "icsp 's ic file numbers: {:?}",
///         icsp::get_ic_file_numbers("identities/identity.pem", "4radi-oqaaa-aaaan-qapwa-cai").await
///     );
/// }
/// ```
pub async fn get_ic_file_numbers(
    pem_identity_path: &str,
    icsp_canister_id_text: &str,
) -> Option<Nat> {
    let canister_id = candid::Principal::from_text(icsp_canister_id_text).unwrap();
    let response_blob = build_agent(pem_identity_path)
        .query(&canister_id, "getIcFileNums")
        .with_arg(Encode!().expect("encode error"))
        .call()
        .await
        .expect("response error");
    Decode!(&response_blob, Option<Nat>).unwrap()
}

/// Slice all files by page_number and return the information of file_info at page_index
///
/// # Examples
///
/// ``` no_run
/// use isp_sdk::icsp;
///
/// pub async fn get_field_file_infos() {
///     let page_num: u64 = 10;
///     let page_index: u64 = 2;
///     println!(
///         "every page have {:?} file_info, query the {:?} page\n",
///         page_num, page_index
///     );
///     let mut index = 0;
///     for file_info in &icsp::get_field_file_infos(
///         "identities/identity.pem",
///         "4radi-oqaaa-aaaan-qapwa-cai",
///         page_num,
///         page_index,
///     )
///         .await
///     {
///         index += 1;
///         println!("the file_info index: {:?}", index);
///         println!("bucket_id: {:?}", file_info.bucket_id.to_text());
///         println!("total_index: {:?}", file_info.total_index);
///         println!("received chunk_number: {:?}", file_info.received);
///         println!("wrote_page: {:?}", file_info.wrote_page);
///         println!("file type: {:?}", file_info.file_type);
///         println!("is_http_open: {:?}", file_info.is_http_open);
///         println!("total_size: {:?}", file_info.total_size);
///         println!("\n");
///     }
/// }
/// ```
pub async fn get_field_file_infos(
    pem_identity_path: &str,
    icsp_canister_id_text: &str,
    page_number: u64,
    page_index: u64,
) -> Vec<FileBufExt> {
    let canister_id = candid::Principal::from_text(icsp_canister_id_text).unwrap();
    let response_blob = build_agent(pem_identity_path)
        .query(&canister_id, "getFieldFileInfos")
        .with_arg(Encode!(&Nat::from(page_number), &Nat::from(page_index)).expect("encode error"))
        .call()
        .await
        .expect("response error");
    Decode!(&response_blob, Vec<FileBufExt>).unwrap()
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

// Access file from file path, slice and return [each slice] array
fn get_file_from_source(path: &str) -> (usize, Vec<Vec<u8>>) {
    let context = fs::read(path).expect("read file failed");
    let size = context.len();
    let slice_size = if context.len() % UPDATE_SIZE == 0 {
        context.len() / UPDATE_SIZE
    } else {
        context.len() / UPDATE_SIZE + 1
    };
    let mut res = Vec::new();
    for index in 0..slice_size {
        if index == slice_size - 1 {
            res.push(context[index * UPDATE_SIZE..context.len()].to_owned())
        } else {
            res.push(context[index * UPDATE_SIZE..(index + 1) * UPDATE_SIZE].to_owned())
        }
    }
    (size, res)
}

fn build_store_args(
    file_key: String,
    file_extension: String,
    total_size: u128,
    data_slice: &Vec<Vec<u8>>,
    is_open: bool,
) -> Vec<StoreArgs> {
    let mut order = 0;
    let mut puts = vec![];
    for data in data_slice {
        puts.push(StoreArgs {
            key: file_key.clone(),
            value: data.to_owned(),
            total_index: Nat::from(data_slice.len() as u128),
            file_type: file_extension.clone(),
            total_size: total_size.clone() as u64,
            is_http_open: is_open,
            index: Nat::from(order.clone()),
        });
        order += 1;
    }
    puts
}

fn get_file_type(file_type: &str) -> &str {
    if file_type == "pdf" {
        return "application/pdf";
    } else if file_type == "jpg" || file_type == "jpeg" {
        return "image/jpg";
    } else if file_type == "png" {
        return "image/png";
    } else if file_type == "mp4" {
        return "video/mp4";
    } else if file_type == "mp3" {
        return "audio/mp3";
    } else if file_type == "gif" {
        return "image/gif";
    } else if file_type == "txt" {
        return "text/plain";
    } else if file_type == "ppt" || file_type == "pptx" {
        return "application/vnd.ms-powerpoint";
    } else if file_type == "html" || file_type == "xhtml" {
        return "text/html";
    } else if file_type == "doc" || file_type == "docx" {
        return "application/msword";
    } else if file_type == "xls" {
        return "application/x-xls";
    } else if file_type == "apk" {
        return "application/vnd.android.package-archive";
    } else if file_type == "svg" {
        return "text/xml";
    } else if file_type == "wmv" {
        return "video/x-ms-wmv";
    } else {
        return "application/octet-stream";
    }
}
