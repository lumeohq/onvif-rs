#[macro_use]
extern crate log;
#[macro_use]
extern crate yaserde_derive;

pub mod schema;

// TODO: disallow dead_code once SOAP is used by HTTP client
#[allow(dead_code)]
mod soap;
