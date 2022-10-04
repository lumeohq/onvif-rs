extern crate onvif;
use onvif::discovery;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    use futures_util::stream::StreamExt;
    const MAX_CONCURRENT_JUMPERS: usize = 100;

    discovery::DiscoveryBuilder::default()
        .run()
        .await
        .unwrap()
        .for_each_concurrent(MAX_CONCURRENT_JUMPERS, |addr| async move {
            println!("Device found: {:?}", addr);
        })
        .await;
}
