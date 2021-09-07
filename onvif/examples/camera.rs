use onvif::{schema, soap};
use structopt::StructOpt;
use tracing::debug;
use url::Url;

#[derive(StructOpt)]
#[structopt(name = "camera", about = "ONVIF camera control tool")]
struct Args {
    #[structopt(global = true, long, requires = "password")]
    username: Option<String>,

    #[structopt(global = true, long, requires = "username")]
    password: Option<String>,

    /// The device's base URI, typically just to the HTTP root.
    /// The service-specific path (such as `/onvif/device_support`) will be appended to this.
    // Note this is an `Option` because global options can't be required in clap.
    // https://github.com/clap-rs/clap/issues/1546
    #[structopt(global = true, long)]
    uri: Option<Url>,

    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(StructOpt)]
#[structopt()]
enum Cmd {
    GetSystemDateAndTime,

    GetCapabilities,

    /// Gets the capabilities of all known ONVIF services supported by this device.
    GetServiceCapabilities,

    /// Gets RTSP URIs for all profiles, along with a summary of the video/audio streams.
    GetStreamUris,

    GetHostname,

    // Gets model, firmware, manufacturer and others informations related to the device.
    GetDeviceInformation,

    SetHostname {
        hostname: String,
    },

    // Gets the PTZ status for the primary media profile.
    GetStatus,

    /// Attempts to enable a `vnd.onvif.metadata` RTSP stream with analytics.
    EnableAnalytics,

    /// Gets information about the currently enabled and supported video analytics.
    GetAnalytics,

    // Try to get any possible information
    GetAll,
}

struct Clients {
    devicemgmt: soap::client::Client,
    event: Option<soap::client::Client>,
    deviceio: Option<soap::client::Client>,
    media: Option<soap::client::Client>,
    media2: Option<soap::client::Client>,
    imaging: Option<soap::client::Client>,
    ptz: Option<soap::client::Client>,
    analytics: Option<soap::client::Client>,
}

impl Clients {
    async fn new(args: &Args) -> Result<Self, String> {
        let creds = match (args.username.as_ref(), args.password.as_ref()) {
            (Some(username), Some(password)) => Some(soap::client::Credentials {
                username: username.clone(),
                password: password.clone(),
            }),
            (None, None) => None,
            _ => panic!("username and password must be specified together"),
        };
        let base_uri = args
            .uri
            .as_ref()
            .ok_or_else(|| "--uri must be specified.".to_string())?;
        let devicemgmt_uri = base_uri.join("onvif/device_service").unwrap();
        let mut out = Self {
            devicemgmt: soap::client::ClientBuilder::new(&devicemgmt_uri)
                .credentials(creds.clone())
                .build(),
            imaging: None,
            ptz: None,
            event: None,
            deviceio: None,
            media: None,
            media2: None,
            analytics: None,
        };
        let services =
            schema::devicemgmt::get_services(&out.devicemgmt, &Default::default()).await?;
        for s in &services.service {
            let url = Url::parse(&s.x_addr).map_err(|e| e.to_string())?;
            if !url.as_str().starts_with(base_uri.as_str()) {
                return Err(format!(
                    "Service URI {} is not within base URI {}",
                    &s.x_addr, &base_uri
                ));
            }
            let svc = Some(
                soap::client::ClientBuilder::new(&url)
                    .credentials(creds.clone())
                    .build(),
            );
            match s.namespace.as_str() {
                "http://www.onvif.org/ver10/device/wsdl" => {
                    if s.x_addr != devicemgmt_uri.as_str() {
                        return Err(format!(
                            "advertised device mgmt uri {} not expected {}",
                            &s.x_addr, &devicemgmt_uri
                        ));
                    }
                }
                "http://www.onvif.org/ver10/events/wsdl" => out.event = svc,
                "http://www.onvif.org/ver10/deviceIO/wsdl" => out.deviceio = svc,
                "http://www.onvif.org/ver10/media/wsdl" => out.media = svc,
                "http://www.onvif.org/ver20/media/wsdl" => out.media2 = svc,
                "http://www.onvif.org/ver20/imaging/wsdl" => out.imaging = svc,
                "http://www.onvif.org/ver20/ptz/wsdl" => out.ptz = svc,
                "http://www.onvif.org/ver20/analytics/wsdl" => out.analytics = svc,
                _ => debug!("unknown service: {:?}", s),
            }
        }
        Ok(out)
    }
}

async fn get_capabilities(clients: &Clients) {
    match schema::devicemgmt::get_capabilities(&clients.devicemgmt, &Default::default()).await {
        Ok(capabilities) => println!("{:#?}", capabilities),
        Err(error) => println!("Failed to fetch capabilities: {}", error.to_string()),
    }
}

async fn get_device_information(clients: &Clients) {
    println!(
        "{:#?}",
        &schema::devicemgmt::get_device_information(&clients.devicemgmt, &Default::default())
            .await
            .unwrap()
    );
}

async fn get_service_capabilities(clients: &Clients) {
    match schema::event::get_service_capabilities(&clients.devicemgmt, &Default::default()).await {
        Ok(capability) => println!("devicemgmt: {:#?}", capability),
        Err(error) => println!("Failed to fetch devicemgmt: {}", error.to_string()),
    }

    if let Some(ref event) = clients.event {
        match schema::event::get_service_capabilities(event, &Default::default()).await {
            Ok(capability) => println!("event: {:#?}", capability),
            Err(error) => println!("Failed to fetch event: {}", error.to_string()),
        }
    }
    if let Some(ref deviceio) = clients.deviceio {
        match schema::event::get_service_capabilities(deviceio, &Default::default()).await {
            Ok(capability) => println!("deviceio: {:#?}", capability),
            Err(error) => println!("Failed to fetch deviceio: {}", error.to_string()),
        }
    }
    if let Some(ref media) = clients.media {
        match schema::event::get_service_capabilities(media, &Default::default()).await {
            Ok(capability) => println!("media: {:#?}", capability),
            Err(error) => println!("Failed to fetch media: {}", error.to_string()),
        }
    }
    if let Some(ref media2) = clients.media2 {
        match schema::event::get_service_capabilities(media2, &Default::default()).await {
            Ok(capability) => println!("media2: {:#?}", capability),
            Err(error) => println!("Failed to fetch media2: {}", error.to_string()),
        }
    }
    if let Some(ref imaging) = clients.imaging {
        match schema::event::get_service_capabilities(imaging, &Default::default()).await {
            Ok(capability) => println!("imaging: {:#?}", capability),
            Err(error) => println!("Failed to fetch imaging: {}", error.to_string()),
        }
    }
    if let Some(ref ptz) = clients.ptz {
        match schema::event::get_service_capabilities(ptz, &Default::default()).await {
            Ok(capability) => println!("ptz: {:#?}", capability),
            Err(error) => println!("Failed to fetch ptz: {}", error.to_string()),
        }
    }
    if let Some(ref analytics) = clients.analytics {
        match schema::event::get_service_capabilities(analytics, &Default::default()).await {
            Ok(capability) => println!("analytics: {:#?}", capability),
            Err(error) => println!("Failed to fetch analytics: {}", error.to_string()),
        }
    }
}

async fn get_system_date_and_time(clients: &Clients) {
    let date =
        schema::devicemgmt::get_system_date_and_time(&clients.devicemgmt, &Default::default())
            .await;
    println!("{:#?}", date);
}

async fn get_stream_uris(clients: &Clients) {
    let media_client = clients.media.as_ref().unwrap();
    let profiles = schema::media::get_profiles(media_client, &Default::default())
        .await
        .unwrap();
    debug!("get_profiles response: {:#?}", &profiles);
    let requests: Vec<_> = profiles
        .profiles
        .iter()
        .map(|p: &schema::onvif::Profile| schema::media::GetStreamUri {
            profile_token: schema::onvif::ReferenceToken(p.token.0.clone()),
            stream_setup: schema::onvif::StreamSetup {
                stream: schema::onvif::StreamType::RtpUnicast,
                transport: schema::onvif::Transport {
                    protocol: schema::onvif::TransportProtocol::Rtsp,
                    tunnel: vec![],
                },
            },
        })
        .collect();

    let responses = futures_util::future::try_join_all(
        requests
            .iter()
            .map(|r| schema::media::get_stream_uri(media_client, r)),
    )
    .await
    .unwrap();
    for (p, resp) in profiles.profiles.iter().zip(responses.iter()) {
        println!("token={} name={}", &p.token.0, &p.name.0);
        println!("    {}", &resp.media_uri.uri);
        if let Some(ref v) = p.video_encoder_configuration {
            println!(
                "    {:?}, {}x{}",
                v.encoding, v.resolution.width, v.resolution.height
            );
            if let Some(ref r) = v.rate_control {
                println!("    {} fps, {} kbps", r.frame_rate_limit, r.bitrate_limit);
            }
        }
        if let Some(ref a) = p.audio_encoder_configuration {
            println!(
                "    audio: {:?}, {} kbps, {} kHz",
                a.encoding, a.bitrate, a.sample_rate
            );
        }
    }
}

async fn get_hostname(clients: &Clients) {
    let resp = schema::devicemgmt::get_hostname(&clients.devicemgmt, &Default::default())
        .await
        .unwrap();
    debug!("get_hostname response: {:#?}", &resp);
    println!(
        "{}",
        resp.hostname_information
            .name
            .as_deref()
            .unwrap_or("(unset)")
    );
}

async fn set_hostname(clients: &Clients, hostname: String) {
    schema::devicemgmt::set_hostname(
        &clients.devicemgmt,
        &schema::devicemgmt::SetHostname { name: hostname },
    )
    .await
    .unwrap();
}

async fn enable_analytics(clients: &Clients) {
    let media_client = clients.media.as_ref().unwrap();
    let mut config = schema::media::get_metadata_configurations(media_client, &Default::default())
        .await
        .unwrap();
    if config.configurations.len() != 1 {
        println!("Expected exactly one analytics config");
        return;
    }
    let mut c = config.configurations.pop().unwrap();
    let token_str = c.token.0.clone();
    println!("{:#?}", &c);
    if c.analytics != Some(true) || c.events.is_none() {
        println!(
            "Enabling analytics in metadata configuration {}",
            &token_str
        );
        c.analytics = Some(true);
        c.events = Some(schema::onvif::EventSubscription {
            filter: None,
            subscription_policy: None,
        });
        schema::media::set_metadata_configuration(
            media_client,
            &schema::media::SetMetadataConfiguration {
                configuration: c,
                force_persistence: true,
            },
        )
        .await
        .unwrap();
    } else {
        println!(
            "Analytics already enabled in metadata configuration {}",
            &token_str
        );
    }

    let profiles = schema::media::get_profiles(media_client, &Default::default())
        .await
        .unwrap();
    let requests: Vec<_> = profiles
        .profiles
        .iter()
        .filter_map(
            |p: &schema::onvif::Profile| match p.metadata_configuration {
                Some(_) => None,
                None => Some(schema::media::AddMetadataConfiguration {
                    profile_token: schema::onvif::ReferenceToken(p.token.0.clone()),
                    configuration_token: schema::onvif::ReferenceToken(token_str.clone()),
                }),
            },
        )
        .collect();
    if !requests.is_empty() {
        println!(
            "Enabling metadata on {}/{} configs",
            requests.len(),
            profiles.profiles.len()
        );
        futures_util::future::try_join_all(
            requests
                .iter()
                .map(|r| schema::media::add_metadata_configuration(media_client, r)),
        )
        .await
        .unwrap();
    } else {
        println!(
            "Metadata already enabled on {} configs",
            profiles.profiles.len()
        );
    }
}

async fn get_analytics(clients: &Clients) {
    let config = schema::media::get_video_analytics_configurations(
        clients.media.as_ref().unwrap(),
        &Default::default(),
    )
    .await
    .unwrap();
    println!("{:#?}", &config);
    let c = match config.configurations.first() {
        Some(c) => c,
        None => return,
    };
    if let Some(ref a) = clients.analytics {
        let mods = schema::analytics::get_supported_analytics_modules(
            a,
            &schema::analytics::GetSupportedAnalyticsModules {
                configuration_token: schema::onvif::ReferenceToken(c.token.0.clone()),
            },
        )
        .await
        .unwrap();
        println!("{:#?}", &mods);
    }
}

async fn get_status(clients: &Clients) {
    if let Some(ref ptz) = clients.ptz {
        let media_client = clients.media.as_ref().unwrap();
        let profile = &schema::media::get_profiles(media_client, &Default::default())
            .await
            .unwrap()
            .profiles[0];
        let profile_token = schema::onvif::ReferenceToken(profile.token.0.clone());
        println!(
            "ptz status: {:#?}",
            &schema::ptz::get_status(ptz, &schema::ptz::GetStatus { profile_token })
                .await
                .unwrap()
        );
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::from_args();
    let clients = Clients::new(&args).await.unwrap();

    match args.cmd {
        Cmd::GetSystemDateAndTime => get_system_date_and_time(&clients).await,
        Cmd::GetCapabilities => get_capabilities(&clients).await,
        Cmd::GetServiceCapabilities => get_service_capabilities(&clients).await,
        Cmd::GetStreamUris => get_stream_uris(&clients).await,
        Cmd::GetHostname => get_hostname(&clients).await,
        Cmd::SetHostname { hostname } => set_hostname(&clients, hostname).await,
        Cmd::GetDeviceInformation => get_device_information(&clients).await,
        Cmd::EnableAnalytics => enable_analytics(&clients).await,
        Cmd::GetAnalytics => get_analytics(&clients).await,
        Cmd::GetStatus => get_status(&clients).await,
        Cmd::GetAll => {
            get_system_date_and_time(&clients).await;
            get_capabilities(&clients).await;
            get_service_capabilities(&clients).await;
            get_stream_uris(&clients).await;
            get_hostname(&clients).await;
            get_analytics(&clients).await;
            get_status(&clients).await;
        }
    }
}
