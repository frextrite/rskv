# rskv

a simple cross-platform key value store gRPC client/server written in rust

## steps to run

### server

`cargo run --bin kvs-server`

### client

`cargo run --bin kvs-client`

## additional information

gRPC communication happens over a unix domain socket on Unix based systems and over TCP on Windows
