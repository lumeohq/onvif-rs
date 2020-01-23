// xsd:
pub mod common;
pub mod metadatastream;
pub mod onvif;
pub mod radiometry;
pub mod rules;
pub mod types;

// wsdl:
pub mod accesscontrol;
pub mod accessrules;
pub mod actionengine;
pub mod advancedsecurity;
pub mod analytics;
pub mod authenticationbehavior;
pub mod b_2;
pub mod bf_2;
pub mod credential;
pub mod deviceio;
pub mod devicemgmt;
pub mod display;
pub mod doorcontrol;
pub mod event;
pub mod imaging;
pub mod media;
pub mod media2;
pub mod provisioning;
pub mod ptz;
pub mod receiver;
pub mod recording;
pub mod replay;
pub mod schedule;
pub mod search;
pub mod t_1;
pub mod thermal;
pub mod uplink;
pub mod ws_addr;

#[cfg(test)]
mod tests;

pub mod transport;
