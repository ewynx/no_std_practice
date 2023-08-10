# Small no-std Rust project

This comes from:
https://os.phil-opp.com/freestanding-rust-binary/

## Build

Compile for a bare metal target such as thumbv7em-none-eabihf
```
rustup target add thumbv7em-none-eabihf
cargo build --target thumbv7em-none-eabihf
```

Alternatively, compule for host system by using following commands:
```
# Linux
cargo rustc -- -C link-arg=-nostartfiles
# Windows
cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
# macOS
cargo rustc -- -C link-args="-e __start -static -nostartfiles"
```