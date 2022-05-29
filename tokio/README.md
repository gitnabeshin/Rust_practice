# Tokio plactice


## setup

```
$ cargo install mini-redis
$ mini-redis-server
$ mini-redis-cli get foo
```

## run

```
$ cargo run
$ cargo run --example hello-redis
```

## Cargo.toml

```
[dependencies]
tokio = { version = "1", features = ["full"] }
mini-redis = "0.4"
bytes = "1"
```

# Reference

* https://tokio.rs/tokio/tutorial
* https://zenn.dev/magurotuna/books/tokio-tutorial-ja


