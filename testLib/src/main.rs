use lib::isp;
use lib::isp::CreateICSPResult;

#[tokio::main]
async fn main() {
    let response_4 = create_icsp("icsp-1", 100_000_000 as u64).await;
    match response_4 {
        CreateICSPResult::ok(pr) => println!("create ok, canister_id:{:?}", pr.to_text()),
        CreateICSPResult::err(er) => println!("{:?}", er),
    }
}

async fn create_icsp(icsp_name: &str, icp_amount: u64) -> CreateICSPResult {
    let response = isp::create_icsp("identities/identity.pem", icsp_name, icp_amount).await;
    response
}
