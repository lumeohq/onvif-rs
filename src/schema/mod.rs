// xsd:
pub mod common;
pub mod duration;
#[allow(unused_imports)]
pub mod metadatastream;
pub mod onvif;
#[allow(unused_imports)]
pub mod radiometry;
#[allow(unused_imports)]
pub mod rules;
pub mod types;

// wsdl:
#[allow(unused_imports)]
pub mod accesscontrol;
#[allow(unused_imports)]
pub mod accessrules;
#[allow(unused_imports)]
pub mod actionengine;
#[allow(unused_imports)]
pub mod advancedsecurity;
#[allow(unused_imports)]
pub mod analytics;
#[allow(unused_imports)]
pub mod authenticationbehavior;
#[allow(unused_imports)]
pub mod b_2;
#[allow(unused_imports)]
pub mod bf_2;
#[allow(unused_imports)]
pub mod credential;
#[allow(unused_imports)]
pub mod deviceio;
#[allow(unused_imports)]
pub mod devicemgmt;
#[allow(unused_imports)]
pub mod display;
#[allow(unused_imports)]
pub mod doorcontrol;
#[allow(unused_imports)]
pub mod event;
#[allow(unused_imports)]
pub mod imaging;
#[allow(unused_imports)]
pub mod media;
#[allow(unused_imports)]
pub mod media2;
#[allow(unused_imports)]
pub mod provisioning;
#[allow(unused_imports)]
pub mod ptz;
#[allow(unused_imports)]
pub mod receiver;
#[allow(unused_imports)]
pub mod recording;
#[allow(unused_imports)]
pub mod replay;
#[allow(unused_imports)]
pub mod schedule;
#[allow(unused_imports)]
pub mod search;
#[allow(unused_imports)]
pub mod t_1;
#[allow(unused_imports)]
pub mod thermal;
#[allow(unused_imports)]
pub mod uplink;
#[allow(unused_imports)]
pub mod ws_addr;

#[cfg(test)]
mod tests;

pub mod transport;
