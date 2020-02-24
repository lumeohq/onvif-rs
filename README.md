# ONVIF-rs

ONVIF-rs is a Rust client library mplementation of the ONVIF specification.

## Installation

## Usage

## Examples
To discover devices on the local network:
```shell script
cargo run --example discovery
```

To get date and time from a known device:
```shell script
cargo run --example camera_datetime -- http://192.168.0.2:8000
```

To set camera hostname using credentials:
```shell script
cargo run --example camera_set_hostname -- http://192.168.0.2:8000 admin qwerty cam2
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](LICENSE)
