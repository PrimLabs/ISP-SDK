use candid::Principal;

#[tokio::main]
async fn main() {
    let response_1 = get_user_icsps().await;
    for i in &response_1 {
        println!("icsp_name:{:?},icsp_canister_id:{:?}\n", i.0, i.1);
    }
    if response_1.is_empty() {
        println!("user do not have icsp\n");
    }

    let response_2 = get_sub_account().await;
    println!("SubAccount:{:?}\n", response_2);

    let response_3 = get_isp_admins().await;
    println!("admins:");
    for i in &response_3 {
        println!("{:?}", Principal::to_text(i));
    }
    println!("\n");
}

// return (icsp_name, icsp_canister_id)
async fn get_user_icsps() -> Vec<(String, Principal)> {
    let response = isp_sdk::get_user_icsps("identities/identity.pem").await;
    response
}

async fn get_sub_account() -> String {
    let response = isp_sdk::get_sub_account("identities/identity.pem").await;
    response
}

async fn get_isp_admins() -> Vec<Principal> {
    let response = isp_sdk::get_isp_admins("identities/identity.pem").await;
    response
}

async fn create_icsp(icsp_name: &str, icp_amount: u64) -> CreateICSPResult {
    let response = isp_sdk::create_icsp("identities/identity.pem", icsp_name, icp_amount).await;
    response
}
