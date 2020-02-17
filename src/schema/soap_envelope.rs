use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

// TODO: replace with actual types generated from .xsd

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
pub struct Envelope {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
pub struct Fault {}
