use candid::{CandidType, Decode, Encode, Nat};
use garcon::Delay;
use ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport;
use ic_agent::{identity::Secp256k1Identity, Agent};
use std::fs::{self};
use std::path::Path;
use uuid::Uuid;
mod icsp_did;
pub use icsp_did::{Buckets, FileBufExt, LiveBucketExt, StoreArgs};

const UPDATE_SIZE: usize = 1992288;

/// Get all ic files 's key from user 's icsp
///
/// Example code :
/// ``` no_run
/// use candid::Nat;
/// use isp_sdk::icsp;
///
/// async fn get_all_ic_file_key(
///     pem_identity_path: &str,
///     icsp_canister_id_text: &str,
/// ) -> Vec<String> {
///     icsp::get_all_ic_file_key("identities/identity.pem", icsp_canister_id_text).await
/// }
///
/// #[tokio::main]
/// async fn main() {
///     println!(
///         "get all ic file key result: {:?}",
///         get_all_ic_file_key("identities/identity.pem", "4radi-oqaaa-aaaan-qapwa-cai").await
///     );
/// }
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
    let response = Decode!(&response_blob, Vec<String>).unwrap();
    response
}

/// Get file's information
///
/// Example code :
/// ``` no_run
/// use candid::Nat;
/// use isp_sdk::icsp::{self, FileBufExt};
///
/// async fn get_file_info(
///     pem_identity_path: &str,
///     icsp_canister_id_text: &str,
///     file_key: String,
/// ) -> Option<FileBufExt> {
///     icsp::get_file_info("identities/identity.pem", icsp_canister_id_text, file_key).await
/// }
///
/// #[tokio::main]
/// async fn main() {
///     println!("get file info result:");
///     match get_file_info(
///         "identities/identity.pem",
///         "5ekwd-fyaaa-aaaan-qaxlq-cai",
///         "14d37b8971e5c73a523de39e0682ba0c08df3a503c49f4f976fe282bc60abfef".to_string(),
///     )
///     .await
///     {
///         None => println!("do not have this file"),
///         Some(file_info) => {
///             println!("bucket_id: {:?}", file_info.bucket_id.to_text());
///             println!("total_index: {:?}", file_info.total_index);
///             println!("received chunk_number: {:?}", file_info.received);
///             println!("wrote_page: {:?}", file_info.wrote_page);
///             println!("file type: {:?}", file_info.file_type);
///             println!("is_http_open: {:?}", file_info.is_http_open);
///             println!("total_size: {:?}", file_info.total_size);
///         }
///     }
/// }
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
    let response = Decode!(&response_blob, Option<FileBufExt>).unwrap();
    response
}

/// Get icsp 's cycle balance
///
/// The cycle balance is e12s
///
/// Example code :
/// ``` no_run
/// use candid::Nat;
/// use isp_sdk::icsp;
///
/// async fn get_cycle_balance(icsp_canister_id_text: &str) -> Nat {
///     icsp::get_cycle_balance("identities/identity.pem", icsp_canister_id_text).await
/// }
///
/// #[tokio::main]
/// async fn main() {
///     println!(
///         "icsp cycle balance:{:?}\n",
///         get_cycle_balance("4radi-oqaaa-aaaan-qapwa-cai").await
///     );
/// }
pub async fn get_cycle_balance(pem_identity_path: &str, icsp_canister_id_text: &str) -> Nat {
    let canister_id = candid::Principal::from_text(icsp_canister_id_text).unwrap();
    let response_blob = build_agent(pem_identity_path)
        .query(&canister_id, "getCycleBalance")
        .with_arg(Encode!().expect("encode piece failed"))
        .call()
        .await
        .expect("response error");
    let response = Decode!(&response_blob, Nat).unwrap();
    response
}

/// Get the bucket where the file is stored
///
/// Example code :
/// ``` no_run
/// use ic_agent::ic_types::Principal;
/// use isp_sdk::icsp::{self};
///
/// async fn get_bucket_of_file(icsp_canister_id_text: &str, file_key: &str) -> Option<Principal> {
///     icsp::get_bucket_of_file("identities/identity.pem", icsp_canister_id_text, file_key).await
/// }
///
/// #[tokio::main]
/// async fn main() {
///     println!(
///         "the file in bucekt:{:?}\n",
///         get_bucket_of_file(
///             "4radi-oqaaa-aaaan-qapwa-cai",
///             "14d37b8971e5c73a523de39e0682ba0c08df3a503c49f4f976fe282bc60abfef",
///         )
///         .await
///         .expect("no bucket have this file")
///         .to_text()
///     );
/// }
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
    let response = Decode!(&response_blob, Option<candid::Principal>).unwrap();
    response
}

/// Get buckets of user's icsp
///
/// Example code :
/// ```no_run
/// use isp_sdk::icsp::{self, Buckets};
///
/// async fn get_icsp_buckets(icsp_canister_id_text: &str) -> Option<Buckets> {
///     icsp::get_icsp_buckets("identities/identity.pem", icsp_canister_id_text).await
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let response = get_icsp_buckets("4radi-oqaaa-aaaan-qapwa-cai").await;
///     match response {
///         Some(response_) => {
///                 println!("old buckets:");
///                 for i in &response_.old_buckets {
///                     println!("{:?}", i.to_text());
///                 }
///                 println!("Live Buckets:");
///                 for i in &response_.live_buckets {
///                     println!(
///                         "canister_id:{:?}, used_memory:{:?}",
///                         i.canister_id.to_text(),
///                         i.used_memory,
///                     );
///                 }
///             },
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
    let response = Decode!(&response_blob, Option<Buckets>).unwrap();
    response
}

/// Get icsp 's admins
///
/// Example code :
/// ``` no_run
/// use ic_agent::ic_types::Principal;
/// use isp_sdk::icsp;
///
/// async fn get_icsp_admins(icsp_canister_id_text: &str) -> Vec<Principal> {
///     icsp::get_icsp_admins("identities/identity.pem", icsp_canister_id_text).await
/// }
///
/// #[tokio::main]
/// async fn main() {
///     println!("icsp admins:");
///     for i in &get_icsp_admins("4radi-oqaaa-aaaan-qapwa-cai").await {
///         println!("{:?}", i.to_text());
///     }
/// }
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
    let response = Decode!(&response_blob, Vec<candid::Principal>).unwrap();
    response
}

/// Store files from folder_path
///
/// If http open,url format: icsp_canister_id.raw.ic0.app/ic/file_key
///
/// Example code :
/// ``` no_run
/// use isp_sdk::icsp;
///
/// async fn store_files(
///     folder_path: &str,
///     icsp_canister_id_text: &str,
///     is_http_open: bool,
/// ) -> Vec<(String, String)> {
///     icsp::store_files(
///         "identities/identity.pem",
///         folder_path,
///         icsp_canister_id_text,
///         is_http_open,
///     ).await
/// }
///
/// #[tokio::main]
/// async fn main() {
///     // url format : icsp_canister_id.raw.ic0.app/'option location'/file_key
///     // icsp_canister_id.raw.ic0.app/ic/file_key
///     // icsp_canister_id.raw.ic0.app/ipfs/file_key
///     // icsp_canister_id.raw.ic0.app/ar/file_key
///     for i in &store_files("source/", "4radi-oqaaa-aaaan-qapwa-cai", true).await {
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
/// Example code :
/// ``` no_run
/// use isp_sdk::icsp;
///
/// async fn store_file(
///     file_path_str: &str,
///     icsp_canister_id_text: &str,
///     is_http_open: bool,
/// ) -> (String, String) {
///     icsp::store_file(
///         "identities/identity.pem",
///         file_path_str,
///         icsp_canister_id_text,
///         is_http_open,
///     ).await
/// }
///
/// #[tokio::main]
/// async fn main() {
///     // url format : icsp_canister_id.raw.ic0.app/'option location'/file_key
///     // icsp_canister_id.raw.ic0.app/ic/file_key
///     // icsp_canister_id.raw.ic0.app/ipfs/file_key
///     // icsp_canister_id.raw.ic0.app/ar/file_key
///     let respoonse =
///         store_file("source/bitcoin.pdf", "4radi-oqaaa-aaaan-qapwa-cai", true).await;
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

/// Store str data
///
/// If http open,url format: icsp_canister_id.raw.ic0.app/ic/file_key
///
/// Example code :
/// ``` no_run
/// use isp_sdk::icsp;
///
/// async fn store_str(data: &str, icsp_canister_id_text: &str, is_http_open: bool) -> String {
///     icsp::store_str(
///         "identities/identity.pem",
///         data,
///         icsp_canister_id_text,
///         is_http_open,
///     ).await
/// }
///
/// #[tokio::main]
/// async fn main() {
///     // url format : icsp_canister_id.raw.ic0.app/'option location'/file_key
///     // icsp_canister_id.raw.ic0.app/ic/file_key
///     // icsp_canister_id.raw.ic0.app/ipfs/file_key
///     // icsp_canister_id.raw.ic0.app/ar/file_key
///     println!(
///         "store_str, file_key: {:?}",
///         store_str(
///             "test_isp_sdk_store_str",
///             "4radi-oqaaa-aaaan-qapwa-cai",
///             true
///         )
///         .await
///     );
/// }
/// ```
pub async fn store_str(
    pem_identity_path: &str,
    data: &str,
    icsp_canister_id_text: &str,
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
    let _response_blob = build_agent(pem_identity_path)
        .update(&canister_id, "store")
        .with_arg(Encode!(&put).expect("encode piece failed"))
        .call_and_wait(get_waiter())
        .await
        .expect("response error");

    file_key
}

/// Get file from icsp, return (data, file_type)
///
/// Example code :
/// ``` no_run
/// use isp_sdk::icsp;
/// use std::fs::OpenOptions;
/// use std::io::Write;
///
/// async fn get_file(icsp_canister_id_text: &str, file_key: &str) -> (Vec<u8>, String) {
///     icsp::get_file("identities/identity.pem", icsp_canister_id_text, file_key).await
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let response = get_file(
///         "4radi-oqaaa-aaaan-qapwa-cai",
///         "3166112af0dcc940f8e7f2199a4200cfb5e2efb40796391201b8fe9e4ff7ca84",
///     )
///     .await;
///     let mut file = std::fs::File::create("output/bitcoin.pdf").expect("create failed");
///     file.write_all(&response.0).expect("write failed");
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
/// Example code :
/// ``` no_run
/// use isp_sdk::icsp;
///
/// async fn add_icsp_admin(icsp_canister_id_text: &str, new_admin_text: &str) {
///     icsp::add_icsp_admin(
///         "identities/identity.pem",
///         icsp_canister_id_text,
///         new_admin_text,
///     ).await
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let response = add_icsp_admin(
///         "4radi-oqaaa-aaaan-qapwa-cai",
///         "bxgws-37y5d-tgmpr-hekbp-y3uxo-yicgs-fo7p3-ccnta-kidrz-74onh-pae",
///     ).await;
/// }
/// ```
pub async fn add_icsp_admin(
    pem_identity_path: &str,
    icsp_canister_id_text: &str,
    new_admin_text: &str,
) {
    let canister_id = candid::Principal::from_text(icsp_canister_id_text).unwrap();
    let new_admin = candid::Principal::from_text(new_admin_text).unwrap();
    let _response_blob = build_agent(pem_identity_path)
        .update(&canister_id, "addAdmin")
        .with_arg(Encode!(&new_admin).expect("encode error"))
        .call_and_wait(get_waiter())
        .await
        .expect("response error");
}

/// Delete admin of icsp
///
/// Example code :
/// ``` no_run
/// use isp_sdk::icsp;
///
/// async fn delete_icsp_admin(icsp_canister_id_text: &str, old_admin_text: &str) {
///     icsp::delete_icsp_admin(
///         "identities/identity.pem",
///         icsp_canister_id_text,
///         old_admin_text,
///     ).await
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let response = delete_icsp_admin(
///         "4radi-oqaaa-aaaan-qapwa-cai",
///         "bxgws-37y5d-tgmpr-hekbp-y3uxo-yicgs-fo7p3-ccnta-kidrz-74onh-pae",
///     ).await;
/// }
/// ```
pub async fn delete_icsp_admin(
    pem_identity_path: &str,
    icsp_canister_id_text: &str,
    old_admin_text: &str,
) {
    let canister_id = candid::Principal::from_text(icsp_canister_id_text).unwrap();
    let old_admin = candid::Principal::from_text(old_admin_text).unwrap();
    let _response_blob = build_agent(pem_identity_path)
        .update(&canister_id, "deleteAdmin")
        .with_arg(Encode!(&old_admin).expect("encode error"))
        .call_and_wait(get_waiter())
        .await
        .expect("response error");
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
