use lib::isp_sdk;

#[tokio::main]
async fn main() {
    let pem_identity_path = "/Users/heyuanxun/Documents/GitHub/ISP-SDK/identity.pem";
    let ans = isp_sdk::get_isp_admins(pem_identity_path).await;
    println!("{:?}", &ans[0].to_text());
}
