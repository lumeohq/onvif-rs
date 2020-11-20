extern crate onvif;
use onvif::discovery;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    use futures::stream::StreamExt;
    const MAX_CONCURRENT_JUMPERS: usize = 100;

    discovery::discover(std::time::Duration::from_secs(1))
        .await
        .unwrap()
        .for_each_concurrent(MAX_CONCURRENT_JUMPERS, |addr| async move {
            println!("Device found at address: {}", addr);
        })
        .await;
}
