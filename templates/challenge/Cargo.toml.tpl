[package]
name = "day##DAY##"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
util = { path = "../../crates/util" }

[lib]
name = "day##DAY##"
path = "src/lib/mod.rs"

[[bin]]
name = "main"
path = "src/bin/main.rs"
