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
                std::env::args()
                    .next()
                    .unwrap_or("camera_get_stream_uri".into())
            );
            std::process::exit(1);
        }
    };

    let client = soap::client::Client::new(&uri, None);
    let profiles = schema::media::get_profiles(&client, &Default::default()).await;
    let stream_uri = schema::media::get_stream_uri(
        &client,
        &schema::media::GetStreamUri {
            profile_token: schema::onvif::ReferenceToken(
                profiles
                    .as_ref()
                    .unwrap()
                    .profiles
                    .first()
                    .unwrap()
                    .token
                    .0
                    .clone(),
            ),
            stream_setup: schema::onvif::StreamSetup {
                stream: schema::onvif::StreamType::RtpUnicast,
                transport: schema::onvif::Transport {
                    protocol: schema::onvif::TransportProtocol::Rtsp,
                    tunnel: vec![],
                },
            },
        },
    )
    .await;

    println!("{:#?}", stream_uri);
}
