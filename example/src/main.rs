mod test_isp;
mod test_isp_certified_log;

#[tokio::main]
async fn main() {
    test_isp_certified_log::test().await;
    test_isp::test().await;
}
