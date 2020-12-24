# Start


## Firstly

1. before
```
git
gcc
clang
curl
```

```
curl https://sh.rustup.rs -sSf | sh
```

**other**

```
rust_init
```

- default:

```
source $HOME/.cargo/env
```

**changed by me**

-  .profile

```
# rust

export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_HOME="$HOME/.cargo/"
export RUSTBINPATH="$HOME/.cargo/bin"
export RUST="$HOME/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu"
export RUST_SRC_PATH="$RUST/lib/rustlib/src/rust/src"
export RUST_BACKTRACE=full
export RUST_LOG=debug
export RLS_ROOT="$HOME/git/rust/tools/rls"
# export RLS_ROOT="$HOME/b/git/rust/tools/rls"
```

**C:\Users\Administrator\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\src**

## Rustup

```
rustup self update
```

```
rustup toolchain install nightly
```

```
rustup component add rust-src
rustup component add rust-docs

```


## cargo

```
cargo install rustfmt

```

```
cargo install racer

```

```
git clone https://github.com/rust-lang-nursery/rls.git
cd rls
rustup override set nightly
cargo build --release

```


```bash
cargo install racer
cargo install rustfmt
cargo install clippy
```
