# rust_axum_mongo_example

## Cmd
```sh
# Rust
$ cargo run # run project
$ cargo --version # check version
$ rustup update # update crates
$ cargo new --bin adder # create main.rs & Cargo.toml
$ cargo new <CRATE_NAME> --lib # create crate
$ cargo test -p xxx # run test, select specific crate

# Docker
$ docker-compose up -d # mongo compass `localhost:8081` &  rust api `localhost:27017`
$ docker-compose exec mongo sh # exec mongo

# MongoDB
$ mongosh -u root -p password # login mongo
```



## Http Request/Response
### health check
```sh
$ curl -X GET -H "Content-Type: application/json" localhost:8080
```
### Ex.Request
```sh
$ curl -X GET -H "Content-Type: application/json" localhost:8080/user/1
```
### Ex.Response
```json
{
    "id":"1",
    "name":"user1"
}
```

### Architecture
1. driver: router, server  
2. app: usecase
3. kernel: domain
4. adapter: query, external

### Docs & Article
1. Rust
    - https://doc.rust-jp.rs/rust-by-example-ja/index.html
    - https://play.rust-lang.org/

2. axum
    - https://zenn.dev/msakuta/articles/83f9991b2aba62
    - https://zenn.dev/codemountains/articles/b3c9c176d821e7
    - https://blog-dry.com/entry/2021/12/26/002649
    - https://crates.io/crates/axum
    - https://powell1213.com/2023/03/02/rust-api-server/

3. layered architecture
    - https://buildersbox.corp-sansan.com/entry/2019/04/21/000000_1
    