extern crate onvif_rs;
use onvif_rs::discovery;

fn main() {
    println!("Searching for devices ...");

    let xaddrs = discovery::discover_sync(std::time::Duration::from_millis(100));

    println!("{:#?}", xaddrs);
}
