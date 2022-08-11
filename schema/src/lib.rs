// Generated code contains upper-case acronyms. Ideally it shouldn't, but changing the codegen
// is not trivial (in addition to changing the casing being a breaking change, of course).
// This issue is being tracked at <https://github.com/lumeohq/xsd-parser-rs/issues/123>.
#![allow(clippy::upper_case_acronyms)]

pub use transport;

// xsd:
pub use common;
#[cfg(feature = "metadatastream")]
pub use metadatastream;
pub use onvif;
#[cfg(feature = "radiometry")]
pub use radiometry;
#[cfg(feature = "rules")]
pub use rules;
pub use soap_envelope;
pub use types;
pub use xmlmime;
pub use xop;

// wsdl:
#[cfg(feature = "accesscontrol")]
pub use accesscontrol;
#[cfg(feature = "accessrules")]
pub use accessrules;
#[cfg(feature = "actionengine")]
pub use actionengine;
#[cfg(feature = "advancedsecurity")]
pub use advancedsecurity;
#[cfg(feature = "analytics")]
pub use analytics;
#[cfg(feature = "authenticationbehavior")]
pub use authenticationbehavior;
pub use b_2;
#[cfg(feature = "bf_2")]
pub use bf_2;
#[cfg(feature = "credential")]
pub use credential;
#[cfg(feature = "deviceio")]
pub use deviceio;
#[cfg(feature = "devicemgmt")]
pub use devicemgmt;
#[cfg(feature = "display")]
pub use display;
#[cfg(feature = "doorcontrol")]
pub use doorcontrol;
#[cfg(feature = "event")]
pub use event;
#[cfg(feature = "imaging")]
pub use imaging;
#[cfg(feature = "media")]
pub use media;
#[cfg(feature = "media2")]
pub use media2;
#[cfg(feature = "provisioning")]
pub use provisioning;
#[cfg(feature = "ptz")]
pub use ptz;
#[cfg(feature = "receiver")]
pub use receiver;
#[cfg(feature = "recording")]
pub use recording;
#[cfg(feature = "replay")]
pub use replay;
#[cfg(feature = "schedule")]
pub use schedule;
#[cfg(feature = "search")]
pub use search;
pub use t_1;
#[cfg(feature = "thermal")]
pub use thermal;
#[cfg(feature = "uplink")]
pub use uplink;
pub use ws_addr;
pub use ws_discovery;
pub use xml_xsd;

#[cfg(test)]
mod tests;
