# ONVIF-rs

ONVIF-rs is a Rust client library implementation of the ONVIF specification.

## Features

- [x] all ONVIF types are generated from official schema
- [ ] all ONVIF operations are generated from official schema
- [x] operations are async (currently only _tokio_ runtime is supported)
- [x] device discovery on the local network using _WS-discovery_ which is mandatory for all ONVIF devices
- [x] authentication using _WS-Security UsernameToken_ which is mandatory for all ONVIF devices
- [x] zero unsafe

## Installation

Cargo.toml:
```toml
[dependencies]
onvif-rs = "0.1"
```

## Examples
To [discover](examples/discovery.rs) devices on the local network:
```shell script
cargo run --example discovery
```

To [get date and time](examples/camera_datetime.rs) from a known device:
```shell script
cargo run --example camera_datetime -- http://192.168.0.2:8000
```

To [set camera hostname](examples/camera_set_hostname.rs) using credentials:
```shell script
cargo run --example camera_set_hostname -- http://192.168.0.2:8000 admin qwerty cam2
```

## Dependencies
- XSD -> Rust code generation: [xsd-parser-rs](https://github.com/lumeohq/xsd-parser-rs)
- XML (de)serialization: [yaserde](https://github.com/media-io/yaserde)

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](LICENSE)
