# rust-tokio-understanding-asynchronous-and-green-thread

[(Zenn, 2022/12) Rust の Tokio で非同期とグリーンスレッドを理解する](https://zenn.dev/tfutada/articles/5e87d6e7131e8e)の写経用リポジトリ

## Environment

- Rust 1.70.0

## Setup

```sh
cargo install tokio-console
```

## Run sample code

```sh
cargo run --example ex1
cargo run --example ex2_async
cargo run --example ex2_sync
cargo run --example ex3_worker1
cargo run --example ex3_worker2
RUSTFLAGS="--cfg tokio_unstable" cargo run --example ex4
cargo run --example ex5
cargo run --example ex6
cargo run --example ex7_customize_tokio_rt
cargo run --example ex8_spawn_blocking
```
