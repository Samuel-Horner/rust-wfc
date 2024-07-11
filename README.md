# rust-wfc

A static implementation of the wave function collapse algorithm I made to replace [this](https://github.com/Samuel-Horner/wfc-c)

NOTE: as I wrote this coming from C, I did not use any of rusts OOP-like features (i.e implementations)

### Usage
```
cargo build --release
./target/release/rust-wfc [WIDTH] [HEIGHT]
```
For example:
```
rust-wfc 50 50
```