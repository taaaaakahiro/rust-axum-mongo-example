# rust-example

## Cmd
```sh
# Rust
$ cargo run # run project
$ cargo --version # check version
$ rustup update # update crates
$ cargo new --bin adder # create crate
$ cargo new add-one --lib # create library crate
$ cargo test -p xxx # run test, select specific crate

# Docker
$ docker-compose exec mongo sh

# MongoDB
$ mongosh -u root -p password
```

## Http Request/Response
### Request
```sh
$ curl -X GET -H "Content-Type: application/json" localhost:8080
$ curl -X GET -H "Content-Type: application/json" localhost:8080/user -d '{"id": "1", "name" : "user1"}'
```
### Response
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
3. layered architecture
    - https://buildersbox.corp-sansan.com/entry/2019/04/21/000000_1
    