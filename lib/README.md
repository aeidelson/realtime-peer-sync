## This is the main library source directory.

### Building (testing / Rust)
Prereqs: Rust

`cargo build`

### Building (Universal iOS libary)
Prereqs: OSX, rustup, cargo-lipo

1. Install rustup: https://github.com/rust-lang-nursery/rustup.rs
2. Add all ios targets
3. `cargo install cargo-lipo`
4. Build the universal library: `cargo lipo`
5. Include the header and universal library in XCode swift project (TODO: Add directions)
