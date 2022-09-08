use lib::isp_sdk;

#[tokio::main]
async fn main() {
    let pem_identity_path = "identity.pem";
    // let ans = isp_sdk::get_isp_admins(pem_identity_path).await;
    // println!("{:?}", &ans[0].to_text());
    let ans = isp_sdk::get_sub_account(pem_identity_path).await;
    println!("{:?}", ans);
}