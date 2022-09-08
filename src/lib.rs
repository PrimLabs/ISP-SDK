pub mod isp_sdk {

    use candid::{CandidType, Decode, Encode, Nat};
    use garcon::Delay;
    use hex::{self};
    use ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport;
    use ic_agent::{ic_types::Principal, identity::Secp256k1Identity, Agent};
    use rayon::prelude::*;
    use serde::Deserialize;
    use sha256::{digest, digest_bytes};
    use std::cell::RefCell;
    use std::fs::{self};

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
        Err(Error),
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

    #[derive(CandidType, Deserialize)]
    pub enum TopUpResult {
        Ok,
        Err(Error),
    }

    #[derive(CandidType, Deserialize)]
    pub struct TopUpArgs {
        icsp_canister_id: Principal,
        icp_amount: u64,
    }

    #[derive(CandidType, Deserialize)]
    struct LiveBucketExt {
        used_memory: Nat,
        canister_id: Principal,
    }

    #[derive(CandidType, Deserialize)]
    struct Buckets {
        old_buckets: Vec<Principal>,
        live_buckets: Vec<LiveBucketExt>,
    }

    // isp ------------------------------------------------------------------------------

    pub async fn get_user_icsps(pem_identity_path: &str) -> Vec<(String, Principal)> {
        let canister_id = Principal::from_text(isp_canister_id_text).unwrap();
        let agent = build_agent(pem_identity_path, canister_id);
        let response_blob = agent
            .query(&canister_id, "getUserICSPs")
            .with_arg(Encode!().expect("encode error"))
            .call()
            .await
            .expect("response error");
        let response = Decode!(&response_blob, Vec<(String, Principal)>).unwrap();
        response
    }

    pub async fn get_sub_account(pem_identity_path: &str) -> String {
        let canister_id = Principal::from_text(isp_canister_id_text).unwrap();
        let agent = build_agent(pem_identity_path, canister_id);
        let response_blob = agent
            .query(&canister_id, "getSubAccount")
            .with_arg(Encode!().expect("encode error"))
            .call()
            .await
            .expect("response error");
        let response = Decode!(&response_blob, Vec<u8>).unwrap();
        hex::encode(response)
    }

    pub async fn get_isp_admins(pem_identity_path: &str) -> Vec<Principal> {
        let canister_id = Principal::from_text(isp_canister_id_text).unwrap();
        let agent = build_agent(pem_identity_path, canister_id);
        let response_blob = agent
            .query(&canister_id, "getAdmins")
            .with_arg(Encode!().expect("encode error"))
            .call()
            .await
            .expect("response error");
        let response = Decode!(&response_blob, Vec<Principal>).unwrap();
        response
    }

    pub async fn create_icsp(
        pem_identity_path: &str,
        icsp_name: &str,
        icp_amount: u64,
    ) -> CreateICSPResult {
        let canister_id = Principal::from_text(isp_canister_id_text).unwrap();
        let agent = build_agent(pem_identity_path, canister_id);
        let waiter = get_waiter();
        let response_blob = agent
            .update(&canister_id, "createICSP")
            .with_arg(Encode!(&icsp_name, &icp_amount).expect("encode error"))
            .call_and_wait(waiter)
            .await
            .expect("response error");
        let response = Decode!(&response_blob, CreateICSPResult).unwrap();
        response
    }

    pub async fn top_up_icsp(pem_identity_path: &str, args: TopUpArgs) -> TopUpResult {
        let canister_id = Principal::from_text(isp_canister_id_text).unwrap();
        let agent = build_agent(pem_identity_path, canister_id);
        let waiter = get_waiter();
        let response_blob = agent
            .update(&canister_id, "topUpICSP")
            .with_arg(Encode!(&args).expect("encode error"))
            .call_and_wait(waiter)
            .await
            .expect("response error");
        let response = Decode!(&response_blob, TopUpResult).unwrap();
        response
    }

    // icsp ------------------------------------------------------------------------------

    pub async fn get_bucket_of_file(
        pem_identity_path: &str,
        icsp_canister_id_text: &str,
        file_key: &str,
    ) -> Option<Principal> {
        let canister_id = Principal::from_text(icsp_canister_id_text).unwrap();
        let agent = build_agent(pem_identity_path, canister_id);
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
        let agent = build_agent(pem_identity_path, canister_id);
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
        let agent = build_agent(pem_identity_path, canister_id);
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
        let agent = build_agent(pem_identity_path, canister_id);
        let waiter = get_waiter();

        let mut ans: Vec<(String, String)> = Vec::new();
        let paths = fs::read_dir(&folder_path).unwrap();
        for path in paths {
            let file_path = path.unwrap().file_name().into_string().unwrap();
            let pos: Vec<&str> = file_path.split(".").collect();
            let file_name = String::from(pos[0]);
            let file_type = String::from(pos[1]);
            let file_extension = String::from(getFileType(&file_type));
            let s = folder_path.to_owned() + &file_path;

            let (file_size, data_slice) = get_file_from_source(&s);

            let puts = build_storeArgs(
                file_name.clone(),
                file_extension,
                file_size.try_into().unwrap(),
                &data_slice,
                is_http_open,
            );
            for put in &puts {
                let response_blob = agent
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

    pub async fn change_bucket_admin(pem_identity_path: &str, icsp_canister_id_text: &str) -> bool {
        let canister_id = Principal::from_text(icsp_canister_id_text).unwrap();
        let agent = build_agent(pem_identity_path, canister_id);
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
        let agent = build_agent(pem_identity_path, canister_id);
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
        let agent = build_agent(pem_identity_path, canister_id);
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

    fn build_agent(pem_identity_path: &str, canister_id: Principal) -> Agent {
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

    fn getFileType(fileType: &str) -> &str {
        if fileType == "pdf" {
            return "application/pdf";
        } else if fileType == "jpg" || fileType == "jpeg" {
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
            return "text/plain";
        } else if fileType == "ppt" || fileType == "pptx" {
            return "application/vnd.ms-powerpoint";
        } else if fileType == "html" || fileType == "xhtml" {
            return "text/html";
        } else if fileType == "doc" || fileType == "docx" {
            return "application/msword";
        } else if fileType == "xls" {
            return "application/x-xls";
        } else if fileType == "apk" {
            return "application/vnd.android.package-archive";
        } else if fileType == "svg" {
            return "text/xml";
        } else if fileType == "wmv" {
            return "video/x-ms-wmv";
        } else {
            return "application/octet-stream";
        }
    }
}
