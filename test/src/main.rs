mod test_ic {

    use candid::Principal;
    use icsp::Buckets;
    use isp::{CreateICSPResult, TopUpArgs, TopUpResult};
    use lib::{icsp, isp};

    pub async fn test() {
        let response_1 = get_user_icsps().await;
        for i in &response_1 {
            println!("icsp_name:{:?},icsp_canister_id:{:?}", i.0, i.1.to_text());
        }
        if response_1.is_empty() {
            println!("user do not have icsp\n");
        }

        let response_2 = get_sub_account().await;
        println!("SubAccount:{:?}\n", response_2);

        let response_3 = get_isp_admins().await;
        println!("isp admins:");
        for i in &response_3 {
            println!("{:?}", Principal::to_text(i));
        }
        println!("\n");

        let response_4 = create_icsp("icsp-1", 20_000_000 as u64).await;
        println!("create icsp result:{:?}\n", response_4);

        let top_up_args = TopUpArgs {
            icsp_canisterId: Principal::from_text("xk2my-yqaaa-aaaal-abdwa-cai").unwrap(),
            icp_amount: 5_000_000 as u64,
        };
        let response_5 = top_up_icsp(top_up_args).await;
        println!("topup icsp result:{:?}\n", response_5);

        let response_6 = get_bucket_of_file(
            "4radi-oqaaa-aaaan-qapwa-cai",
            "219ae72471e1857546a9311079c3c02750b15c9e29179498658ba7b5324dd2a5",
        )
        .await;
        match response_6 {
            Some(response) => println!("the file in bucekt:{:?}\n", response.to_text()),
            None => println!("do not have this file"),
        }

        let response_7 = get_icsp_buckets("4radi-oqaaa-aaaan-qapwa-cai").await;
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

        let response_8 = get_icsp_admins("4radi-oqaaa-aaaan-qapwa-cai").await;
        println!("icsp admins:");
        for i in &response_8 {
            println!("{:?}", i.to_text());
        }

        // url format : icsp_canister_id.raw.ic0.app/file_key
        let response_9 = store_file("source/", "4radi-oqaaa-aaaan-qapwa-cai", true).await;
        for i in &response_9 {
            println!("file_name:{:?},file_key:{:?}", i.0, i.1);
        }

        let response_10 = get_file(
            "4radi-oqaaa-aaaan-qapwa-cai",
            "efb8933d26461d4a00bd28824e64d52ff11ebaa6a3584b2478c7f8c0e89b3c8c",
        )
        .await;
        println!("file:{:?},file_type:{:?}", response_10.0, response_10.1);

        let response_11 = change_bucket_admin("4radi-oqaaa-aaaan-qapwa-cai").await;
        println!("change bucket admin result:{:?}", response_11);

        let response_12 = add_icsp_admin(
            "4radi-oqaaa-aaaan-qapwa-cai",
            "bxgws-37y5d-tgmpr-hekbp-y3uxo-yicgs-fo7p3-ccnta-kidrz-74onh-pae",
        )
        .await;
        println!("add icsp admin result:{:?}", response_12);

        let response_13 = change_icsp_admin(
            "4radi-oqaaa-aaaan-qapwa-cai",
            vec![
                "5gdgj-5vp3h-a4vts-zlfdz-oqoan-t6gbc-nh7eo-oj33d-pgesh-wcvb4-sqe",
                "rqtm7-blweq-njir5-hqz4o-lmz7w-zap72-64cug-eqe7x-aryqg-bvwib-zqe",
            ],
        )
        .await;
        println!("change icsp admin result:{:?}", response_13);
    }

    // return (icsp_name, icsp_canister_id)
    async fn get_user_icsps() -> Vec<(String, Principal)> {
        let response = isp::get_user_icsps("identities/identity.pem").await;
        response
    }

    async fn get_sub_account() -> String {
        let response = isp::get_sub_account("identities/identity.pem").await;
        response
    }

    async fn get_isp_admins() -> Vec<Principal> {
        let response = isp::get_isp_admins("identities/identity.pem").await;
        response
    }

    async fn create_icsp(icsp_name: &str, icp_amount: u64) -> CreateICSPResult {
        let response = isp::create_icsp("identities/identity.pem", icsp_name, icp_amount).await;
        response
    }

    async fn top_up_icsp(args: TopUpArgs) -> TopUpResult {
        let response = isp::top_up_icsp("identities/identity.pem", args).await;
        response
    }

    async fn get_bucket_of_file(icsp_canister_id_text: &str, file_key: &str) -> Option<Principal> {
        let response =
            icsp::get_bucket_of_file("identities/identity.pem", icsp_canister_id_text, file_key)
                .await;
        response
    }

    async fn get_icsp_buckets(icsp_canister_id_text: &str) -> Option<Buckets> {
        let response =
            icsp::get_icsp_buckets("identities/identity.pem", icsp_canister_id_text).await;
        response
    }

    async fn get_icsp_admins(icsp_canister_id_text: &str) -> Vec<Principal> {
        let response =
            icsp::get_icsp_admins("identities/identity.pem", icsp_canister_id_text).await;
        response
    }

    async fn store_file(
        folder_path: &str,
        icsp_canister_id_text: &str,
        is_http_open: bool,
    ) -> Vec<(String, String)> {
        let response = icsp::store_file(
            "identities/identity.pem",
            folder_path,
            icsp_canister_id_text,
            is_http_open,
        )
        .await;
        response
    }

    async fn get_file(icsp_canister_id_text: &str, file_key: &str) -> (Vec<u8>, String) {
        let response =
            icsp::get_file("identities/identity.pem", icsp_canister_id_text, file_key).await;
        response
    }

    async fn change_bucket_admin(icsp_canister_id_text: &str) -> bool {
        let response =
            icsp::change_bucket_admin("identities/identity.pem", icsp_canister_id_text).await;
        response
    }

    async fn add_icsp_admin(icsp_canister_id_text: &str, new_admin_text: &str) -> bool {
        let response = icsp::add_icsp_admin(
            "identities/identity.pem",
            icsp_canister_id_text,
            new_admin_text,
        )
        .await;
        response
    }

    async fn change_icsp_admin(icsp_canister_id_text: &str, new_admins_text: Vec<&str>) -> bool {
        let response = icsp::change_icsp_admin(
            "identities/identity.pem",
            icsp_canister_id_text,
            new_admins_text,
        )
        .await;
        response
    }
}

#[tokio::main]
async fn main() {
    test_ic::test().await;
}
