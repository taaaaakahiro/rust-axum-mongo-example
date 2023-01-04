# rust-example

### cmd
```rust
$ cargo run
$ cargo --version
$ rustup update
```
```mongo
$ docker-compose exec mongo sh
$ mongosh -u root -p password
```

### Axum
 - https://github.com/tokio-rs/axum

```sh
$ curl -X GET -H "Content-Type: application/json" localhost:8080
$ curl -X POST -H "Content-Type: application/json" localhost:8080/users -d '{"username" : "user1"}' 

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
    - https://blog-dry.com/entry/2021/12/26/002649
    - https://crates.io/crates/axum
3. layered architecture
    - https://buildersbox.corp-sansan.com/entry/2019/04/21/000000_1
    