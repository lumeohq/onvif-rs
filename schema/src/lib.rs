#[macro_use]
extern crate yaserde_derive;

pub mod transport;
pub mod validate;

// xsd:
pub mod common;
#[allow(clippy::large_enum_variant)]
pub mod metadatastream;
pub mod onvif;
pub mod radiometry;
pub mod rules;
pub mod soap_envelope;
pub mod types;
pub mod xmlmime;
pub mod xop;

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
pub mod ws_discovery;
pub mod xml_xsd;

#[cfg(test)]
mod tests;
mod utils;
