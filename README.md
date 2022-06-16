# Cal-Server

Server implementation for [CAL](https://github.com/johnyenter-briars/cal) - the simple calendar app. This program maintains data and serves requests to users who have a valid API key and account setup in the CAL system.

### Built With

* [Actix-Web](https://crates.io/crates/actix-web)
* [Rusqlite](https://crates.io/crates/rusqlite)

## Getting Started

TODO

### Prerequisites for Running From Source

[Rust](https://www.rust-lang.org/) tooling installed.

### Running Cal-Server
Clone the source code and navigate to the root directory, and run:
```sh
cargo run -- <args>
```
If running over the Web (default), navigate to a browser and then go to either [localhost:8000](http://localhost:8080) or [127.0.0.1:8000](http://127.0.0.1:8080)

## Usage

The binary currently supports several command line arguments.

| Flag | Name | Description |
| ----------- | ----------- | ----------- |
| -h | --help |                 Print help information
| -i | --ip <IP> |              Ip Address the application should bind to [default: 127.0.0.1]
| -p | --port <PORT> |         Port for the application should bind to [default: 8080]
| -r | --reset-db |             Whether or not to delete and refresh the database before starup
| -t | --test-data |           Populate the database with test data
| -d | --delete-old-saves |           Database should delete the old save file when it's loaded into the database
| -u | --user-id <USER_ID> |    UserId for root user
| -V | --version |              Print version information

For a complete description on the arguments, run using the `--help` flag.

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Contact

John Yenter-Briars - <jyenterbriars@gmail.com>

Project Link: [https://github.com/johnyenter-briars/chrust](https://github.com/johnyenter-briars/cal-server)


## Acknowledgements

* [The Good People of the Rust Discord](https://discord.com/invite/rust)
