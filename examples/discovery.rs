extern crate onvif_rs;
use async_std::stream::StreamExt;
use onvif_rs::discovery;

fn main() {
    println!("Searching for devices ...");

    let xaddrs = async_std::task::block_on(async {
        discovery::discover(std::time::Duration::from_millis(100))
            .await
            .unwrap()
            .collect::<Vec<_>>()
            .await
    });

    println!("Devices found: {:#?}", xaddrs);
}
