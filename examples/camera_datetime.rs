extern crate onvif_rs;
use onvif_rs::{schema, soap};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let uri = match std::env::args().skip(1).next() {
        Some(arg) => arg,
        None => {
            eprintln!(
                "Usage: {} camera_uri",
                std::env::args().next().unwrap_or("camera_datetime".into())
            );
            std::process::exit(1);
        }
    };

    let mut client = soap::client::Client::new(&uri, None);
    let date = schema::devicemgmt::get_system_date_and_time(&mut client, &Default::default()).await;

    println!("{:#?}", date);
}
