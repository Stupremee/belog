# Belog

[![Crates.io](https://img.shields.io/crates/v/belog)](https://crates.io/crates/belog)

**Be**autiful **log** is a minimal and pretty logging implementation for the [log](https://crates.io/crates/log) crate.

## Preview

[![asciicast](https://asciinema.org/a/y4BiQwhmFw4FnWGEuPOOZNEl6.svg)](https://asciinema.org/a/y4BiQwhmFw4FnWGEuPOOZNEl6)

## Usage

Add this to your Cargo.toml:
```toml
[dependencies]
belog = "0.1.0"
```

Then you can use the log crate:
```rust
#[macro_use]
extern crate log;

fn main() {
	// Initializes the logger with the max_level set to info.
	belog::init();

	error!("something went wrong. pls fix");
	// this will print nothing, because debug is lower than info.
	debug!("debug info: {}", 1);
}
```

### Enable colored output

To enable colored output, you have to enable the `colored` feature in the belog dependency.
```toml
[dependencies.blog]
version = "0.1.0"
features = ["colored"]
```

## License

This project is licensed under the GPL v3 license.
