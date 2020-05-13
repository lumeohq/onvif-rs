extern crate onvif_rs;
use onvif_rs::{schema, soap};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let uri = match std::env::args().nth(1) {
        Some(arg) => arg,
        None => {
            eprintln!(
                "Usage: {} camera_uri",
                std::env::args()
                    .next()
                    .unwrap_or_else(|| "camera_datetime".into())
            );
            std::process::exit(1);
        }
    };

    let client = soap::client::Client::new(&uri, None);
    let date = schema::devicemgmt::get_system_date_and_time(&client, &Default::default()).await;

    println!("{:#?}", date);
}
