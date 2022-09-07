pub mod isp_sdk {
    use std::cell::{RefCell};
    use ic_agent::{Agent, ic_types::Principal, identity::Secp256k1Identity};
    use ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport;
    use candid::{Encode, Decode, CandidType, Nat};
    use serde::Deserialize;
    use std::fs::{self};
    use rayon::prelude::*;
    use sha256::{digest, digest_bytes};

    const UPDATE_SIZE: usize = 1992288;
    static isp_canister_id_text: &'static str = "4radi-oqaaa-aaaan-qapwa-cai";

    #[derive(CandidType, Deserialize)]
    pub enum Error {
        CreateCanisterFailed(Nat),
        LedgerTransferFailed(Nat),
        Unauthorized,
    }

    #[derive(CandidType, Deserialize)]
    pub enum CreateICSPResult {
        Ok(Principal),
        Err(Error)
    }

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

    pub async fn create_a_icsp(pem_identity_path: &str, icsp_name: &str, icp_amount: u64) -> Principal {
        let url = "https://ic0.app".to_string();
        let identity= Secp256k1Identity::from_pem_file(String::from(pem_identity_path)).unwrap();
        let isp_canister_id = Principal::from_text(isp_canister_id_text).unwrap();
        let transport = ReqwestHttpReplicaV2Transport::create(url).expect("transport error");
        let agent = Agent::builder()
            .with_transport(transport)
            .with_identity(identity)
            .build().expect("build agent error");
        let waiter = garcon::Delay::builder()
            .throttle(std::time::Duration::from_millis(500))
            .timeout(std::time::Duration::from_secs(60 * 5))
            .build();
        let response_blob = agent
            .update(&isp_canister_id, "createICSP")
            .with_arg(Encode!(&icsp_name, &icp_amount).expect("encode error"))
            .call_and_wait(waiter)
            .await.expect("response error");
        let response = Decode!(&response_blob, CreateICSPResult).unwrap();
        match response {
            CreateICSPResult::Ok(icsp_canister_id) => return icsp_canister_id,
            _ => {}
        }
        Principal::from_text("aaaaa-aa").unwrap()
    }

    pub async fn get_user_icsps(pem_identity_path: &str) -> Vec<(String, Principal)> {
        let url = "https://ic0.app".to_string();
        let identity= Secp256k1Identity::from_pem_file(String::from(pem_identity_path)).unwrap();
        let isp_canister_id = Principal::from_text(isp_canister_id_text).unwrap();
        let transport = ReqwestHttpReplicaV2Transport::create(url).expect("transport error");
        let agent = Agent::builder()
            .with_transport(transport)
            .with_identity(identity)
            .build().expect("build agent error");
        let response_blob = agent
            .query(&isp_canister_id, "getUserICSPs")
            .with_arg(Encode!().expect("encode error"))
            .call()
            .await.expect("response error");
        let response = Decode!(&response_blob, Vec<(String, Principal)>).unwrap();
        response
    }

    pub async fn store_file(pem_identity_path: &str, folder_path: &str,icsp_canister_id_text: &str) -> Vec<(String, String)> {
        let url = "https://ic0.app".to_string();
        let identity= Secp256k1Identity::from_pem_file(String::from(pem_identity_path)).unwrap();
        let icsp_canister_id = Principal::from_text(icsp_canister_id_text).unwrap();
        let transport = ReqwestHttpReplicaV2Transport::create(url).expect("transport error");
        let agent = Agent::builder()
            .with_transport(transport)
            .with_identity(identity)
            .build().expect("build agent error");
        let waiter = garcon::Delay::builder()
            .throttle(std::time::Duration::from_millis(500))
            .timeout(std::time::Duration::from_secs(60 * 5))
            .build();

        let mut ans: Vec<(String,String)> = Vec::new();
        let paths = fs::read_dir(&folder_path).unwrap();
        for path in paths {
            let file_path = path.unwrap().file_name().into_string().unwrap();
            let pos :Vec<&str> = file_path.split(".").collect();
            let file_name = String::from(pos[0]);
            let file_type = String::from(pos[1]);
            let file_extension = String::from(getFileType(&file_type));
            let s = folder_path.to_owned() + &file_path;

            let (file_size, data_slice) = get_file_from_source(&s);

            let puts = build_storeArgs(file_name.clone(), file_extension, file_size.try_into().unwrap(), &data_slice);
            for put in &puts {
                let response = agent
                    .update(&icsp_canister_id, "store")
                    .with_arg(Encode!(put).expect("encode piece failed"))
                    .call_and_wait(waiter.clone())
                    .await.expect("response error");
            }
            ans.push((file_name.clone(), puts[0].key.clone()));
        }
        ans
    }

    pub async fn get_isp_admins(pem_identity_path: &str) -> Vec<Principal> {
        let url = "https://ic0.app".to_string();
        let identity= Secp256k1Identity::from_pem_file(String::from(pem_identity_path)).unwrap();
        let isp_canister_id = Principal::from_text(isp_canister_id_text).unwrap();
        let transport = ReqwestHttpReplicaV2Transport::create(url).expect("transport error");
        let agent = Agent::builder()
            .with_transport(transport)
            .with_identity(identity)
            .build().expect("build agent error");
        let response_blob = agent
            .query(&isp_canister_id, "getAdmins")
            .with_arg(Encode!().expect("encode error"))
            .call()
            .await.expect("response error");
        let response = Decode!(&response_blob, Vec<Principal>).unwrap();
        response
    }

    // 从文件路径访问文件，切片并且返回 [每一片] 数组
    fn get_file_from_source(path: &str) -> (usize, Vec<Vec<u8>>) {
        let context = fs::read(path).expect("read file failed");
        let size = context.len();
        println!("file size : {}", context.len());
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
        println!("file chunk number : {}", res.len());
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
        let mut index = 0;
        for bytes in digests {
            for byte in bytes {
                digest.push(*byte);
                index += 1;
            }
        }
        digest_bytes(&digest)
    }

    fn build_storeArgs(
        file_name: String,
        file_extension: String,
        total_size: u128,
        data_slice: &Vec<Vec<u8>>,
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
                is_http_open: true,
                index: Nat::from(order.clone()),
            });
            order += 1;
        }
        puts
    }

    fn getFileType(fileType: &str) -> &str {
        if fileType == "pdf" {
            return "application/pdf";
        } else if fileType == "jpg" {
            return "image/jpg";
        } else if fileType == "png" {
            return "image/png";
        } else if fileType == "mp4" {
            return "video/mp4";
        } else if fileType == "mp3" {
            return "audio/mp3";
        } else if fileType == "gif" {
            return "image/gif";
        } else if fileType == "txt" {
            return "text/html";
        } else if fileType == "ppt" {
            return "application/x-ppt";
        } else {
            return "application/octet-stream";
        }
    }

}
