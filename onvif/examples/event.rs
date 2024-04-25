// This example pulls messages related to the RuleEngine topic.
// RuleEngine topic consists of events related to motion detection.
// Tested on Dahua, uniview, reolink and axis ip cameras.
// Don't forget to set the camera's IP address, username and password.

use onvif::soap::client::{ClientBuilder, Credentials};
use schema::event::{self, CreatePullPointSubscription, PullMessages};
use url::Url;

#[derive(Debug, Clone)]
pub struct Camera {
    pub device_service_url: String,
    pub username: String,
    pub password: String,
    pub event_service_url: String,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            device_service_url: "http://192.168.1.100/onvif/device_service".to_string(),
            username: "admin".to_string(),
            password: "admin".to_string(),
            event_service_url: "http://192.168.1.100/onvif/event_service".to_string(),
        }
    }
}

#[tokio::main]
async fn main() {
    let camera_ip = "192.168.1.50";
    let username = "admin";
    let password = "admin";

    let camera: Camera = Camera {
        device_service_url: format!("http://{}/onvif/device_service", camera_ip),
        username: username.to_string(),
        password: password.to_string(),
        event_service_url: format!("http://{}/onvif/event_service", camera_ip),
    };

    let creds: Credentials = Credentials {
        username: camera.username.to_string(),
        password: camera.password.to_string(),
    };
    let event_client = ClientBuilder::new(&Url::parse(&camera.event_service_url).unwrap())
        .credentials(Some(creds))
        .build();
    let create_pull_sub_request = CreatePullPointSubscription {
        initial_termination_time: None,
        filter: Some(b_2::FilterType {
            topic_expression: Some(b_2::TopicExpressionType {
                dialect: "http://www.onvif.org/ver10/tev/topicExpression/ConcreteSet".to_string(),
                inner_text: "tns1:RuleEngine//.".to_string(),
            }),
        }),
        subscription_policy: None,
    };
    let create_pull_puint_sub_response =
        event::create_pull_point_subscription(&event_client, &create_pull_sub_request).await;
    let camera_sub = match create_pull_puint_sub_response {
        Ok(sub) => sub,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };

    let uri: Url = Url::parse(&camera_sub.subscription_reference.address).unwrap();
    let creds: Credentials = Credentials {
        username: camera.username.to_string(),
        password: camera.password.to_string(),
    };
    let pull_msg_client = ClientBuilder::new(&uri)
        .credentials(Some(creds))
        .auth_type(onvif::soap::client::AuthType::Digest)
        .build();
    let pull_messages_request = PullMessages {
        message_limit: 256,
        timeout: xsd_types::types::Duration {
            seconds: 1.0,
            ..Default::default()
        },
    };

    // Main Loop
    loop {
        let pull_messages_response =
            event::pull_messages(&pull_msg_client, &pull_messages_request).await;
        let msg = match pull_messages_response {
            Ok(msg) => msg,
            Err(e) => {
                println!("Error: {:?}", e);
                continue;
            }
        };
        if !msg.notification_message.is_empty() {
            println!("Notification Message: {:?}", msg.notification_message[0]);
        } else {
            println!("No new notification message");
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    }
}
