use candid::{CandidType, Decode, Encode, Nat};
use garcon::Delay;
use ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport;
use ic_agent::{ic_types::Principal, identity::Secp256k1Identity, Agent};
use rayon::prelude::*;
use serde::Deserialize;
use sha256::digest_bytes;
use std::fs::{self};

#[derive(CandidType, Deserialize)]
pub struct StoreArgs {
    key: String,
    value: Vec<u8>,
    total_index: Nat,
    file_type: String,
    is_http_open: bool,
    total_size: u64,
    index: Nat,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct LiveBucketExt {
    pub used_memory: Nat,
    pub canister_id: Principal,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct Buckets {
    pub old_buckets: Vec<Principal>,
    pub live_buckets: Vec<LiveBucketExt>,
}

const UPDATE_SIZE: usize = 1992288;

pub async fn get_bucket_of_file(
    pem_identity_path: &str,
    icsp_canister_id_text: &str,
    file_key: &str,
) -> Option<Principal> {
    let canister_id = Principal::from_text(icsp_canister_id_text).unwrap();
    let agent = build_agent(pem_identity_path);
    let response_blob = agent
        .query(&canister_id, "getBucketOfFile")
        .with_arg(Encode!(&file_key).expect("encode piece failed"))
        .call()
        .await
        .expect("response error");
    let response = Decode!(&response_blob, Option<Principal>).unwrap();
    response
}

pub async fn get_icsp_buckets(
    pem_identity_path: &str,
    icsp_canister_id_text: &str,
) -> Option<Buckets> {
    let canister_id = Principal::from_text(icsp_canister_id_text).unwrap();
    let agent = build_agent(pem_identity_path);
    let response_blob = agent
        .query(&canister_id, "getBuckets")
        .with_arg(Encode!().expect("encode piece failed"))
        .call()
        .await
        .expect("response error");
    let response = Decode!(&response_blob, Option<Buckets>).unwrap();
    response
}

pub async fn get_icsp_admins(
    pem_identity_path: &str,
    icsp_canister_id_text: &str,
) -> Vec<Principal> {
    let canister_id = Principal::from_text(icsp_canister_id_text).unwrap();
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

pub async fn store_file(
    pem_identity_path: &str,
    folder_path: &str,
    icsp_canister_id_text: &str,
    is_http_open: bool,
) -> Vec<(String, String)> {
    let canister_id = Principal::from_text(icsp_canister_id_text).unwrap();
    let agent = build_agent(pem_identity_path);
    let waiter = get_waiter();

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

        let puts = build_store_args(
            file_extension,
            file_size.try_into().unwrap(),
            &data_slice,
            is_http_open,
        );
        for put in &puts {
            let _response_blob = agent
                .update(&canister_id, "store")
                .with_arg(Encode!(put).expect("encode piece failed"))
                .call_and_wait(waiter.clone())
                .await
                .expect("response error");
        }
        ans.push((file_name.clone(), puts[0].key.clone()));
    }
    ans
}

pub async fn get_file(
    pem_identity_path: &str,
    icsp_canister_id_text: &str,
    file_key: &str,
) -> (Vec<u8>, String) {
    let bucket_canister_id = get_bucket_of_file(pem_identity_path, icsp_canister_id_text, file_key)
        .await
        .expect("can not find bucket have this file");
    let agent = build_agent(pem_identity_path);
    let waiter = get_waiter();

    let total_index_blob = agent
        .update(&bucket_canister_id, "getFileTotalIndex")
        .with_arg(Encode!(&file_key).expect("encode failed"))
        .call_and_wait(waiter)
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
            .query(&bucket_canister_id, "get")
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

pub async fn change_bucket_admin(pem_identity_path: &str, icsp_canister_id_text: &str) -> bool {
    let canister_id = Principal::from_text(icsp_canister_id_text).unwrap();
    let agent = build_agent(pem_identity_path);
    let waiter = get_waiter();
    let response_blob = agent
        .update(&canister_id, "change_bucket_admin")
        .with_arg(Encode!().expect("encode error"))
        .call_and_wait(waiter)
        .await
        .expect("response error");
    let response = Decode!(&response_blob, bool).unwrap();
    response
}

pub async fn add_icsp_admin(
    pem_identity_path: &str,
    icsp_canister_id_text: &str,
    new_admin_text: &str,
) -> bool {
    let canister_id = Principal::from_text(icsp_canister_id_text).unwrap();
    let agent = build_agent(pem_identity_path);
    let waiter = get_waiter();
    let new_admin = Principal::from_text(new_admin_text).unwrap();
    let response_blob = agent
        .update(&canister_id, "addAdmin")
        .with_arg(Encode!(&new_admin).expect("encode error"))
        .call_and_wait(waiter)
        .await
        .expect("response error");
    let response = Decode!(&response_blob, bool).unwrap();
    response
}

pub async fn change_icsp_admin(
    pem_identity_path: &str,
    icsp_canister_id_text: &str,
    new_admins_text: Vec<&str>,
) -> bool {
    let canister_id = Principal::from_text(icsp_canister_id_text).unwrap();
    let agent = build_agent(pem_identity_path);
    let waiter = get_waiter();
    let mut new_admins: Vec<Principal> = Vec::new();
    for i in new_admins_text {
        new_admins.push(Principal::from_text(i).unwrap());
    }
    let response_blob = agent
        .update(&canister_id, "changeAdmin")
        .with_arg(Encode!(&new_admins).expect("encode error"))
        .call_and_wait(waiter)
        .await
        .expect("response error");
    let response = Decode!(&response_blob, bool).unwrap();
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

fn get_file_sha256_digest(context: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut digests = vec![vec![0x00 as u8]; context.len()];
    let mut contents = digests.iter_mut().zip(context.iter()).collect::<Vec<_>>();
    contents
        .par_iter_mut()
        .for_each(|(d, text)| **d = digest_bytes(*text).into_bytes()[..32].to_vec());
    digests
}

fn get_file_key(digests: &Vec<Vec<u8>>) -> String {
    let mut digest = vec![0x00 as u8; 32 * digests.len()];
    let mut _index = 0;
    for bytes in digests {
        for byte in bytes {
            digest.push(*byte);
            _index += 1;
        }
    }
    digest_bytes(&digest)
}

fn build_store_args(
    file_extension: String,
    total_size: u128,
    data_slice: &Vec<Vec<u8>>,
    is_open: bool,
) -> Vec<StoreArgs> {
    let mut order = 0;
    let mut puts = vec![];
    let file_key = get_file_key(&get_file_sha256_digest(data_slice));
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
