// Generated code contains upper-case acronyms. Ideally it shouldn't, but changing the codegen
// is not trivial (in addition to changing the casing being a breaking change, of course).
// This issue is being tracked at <https://github.com/lumeohq/xsd-parser-rs/issues/123>.
#![allow(clippy::upper_case_acronyms)]

#[macro_use]
extern crate yaserde_derive;

pub mod transport;
pub mod validate;

// xsd:
pub mod common;
#[cfg(feature = "metadatastream")]
#[allow(clippy::large_enum_variant)]
pub mod metadatastream;
pub mod onvif;
#[cfg(feature = "radiometry")]
pub mod radiometry;
#[cfg(feature = "rules")]
pub mod rules;
pub mod soap_envelope;
pub mod types;
pub mod xmlmime;
pub mod xop;

// wsdl:
#[cfg(feature = "accesscontrol")]
pub mod accesscontrol;
#[cfg(feature = "accessrules")]
pub mod accessrules;
#[cfg(feature = "actionengine")]
pub mod actionengine;
#[cfg(feature = "advancedsecurity")]
pub mod advancedsecurity;
#[cfg(feature = "analytics")]
pub mod analytics;
#[cfg(feature = "authenticationbehavior")]
pub mod authenticationbehavior;
pub mod b_2;
#[cfg(feature = "bf_2")]
pub mod bf_2;
#[cfg(feature = "credential")]
pub mod credential;
#[cfg(feature = "deviceio")]
pub mod deviceio;
#[cfg(feature = "devicemgmt")]
pub mod devicemgmt;
#[cfg(feature = "display")]
pub mod display;
#[cfg(feature = "doorcontrol")]
pub mod doorcontrol;
#[cfg(feature = "event")]
pub mod event;
#[cfg(feature = "imaging")]
pub mod imaging;
#[cfg(feature = "media")]
pub mod media;
#[cfg(feature = "media2")]
pub mod media2;
#[cfg(feature = "provisioning")]
pub mod provisioning;
#[cfg(feature = "ptz")]
pub mod ptz;
#[cfg(feature = "receiver")]
pub mod receiver;
#[cfg(feature = "recording")]
pub mod recording;
#[cfg(feature = "replay")]
pub mod replay;
#[cfg(feature = "schedule")]
pub mod schedule;
#[cfg(feature = "search")]
pub mod search;
pub mod t_1;
#[cfg(feature = "thermal")]
pub mod thermal;
#[cfg(feature = "uplink")]
pub mod uplink;
pub mod ws_addr;
pub mod ws_discovery;
pub mod xml_xsd;

#[cfg(test)]
mod tests;
mod utils;
