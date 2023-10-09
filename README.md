# violet-sdk

A CS:GO SDK originally used in a private project of mine made with [rust-lang.](https://github.com/rust-lang/rust) If you want to try this out, read the [instructions below](#installation).
<br>
This is not currently working, but maybe it helps people to make their own SDK in Rust, this project is like 2 years old and I have no use for it
anymore.
If you have improvement ideas, discovered bugs or just want to contact me for whatever reason here's my discord:
`felixfem`


# Installation

Just add the following to your project's Cargo.toml file:

```toml
[dependencies]
sdk = { version = "^1.0", git = "https://github.com/felix-rs/csgo-sdk" }
```

# Usage

After you have added the crate to your Cargo.toml you need to initialize the SDK once, somewhere in your DLL.
For that just call ```sdk::initialize();``` (this returns a result you need to handle).
Now you can call ```sdk::get_interfaces()``` to get access to all the interfaces and their virtual functions etc.

# Example

```rust
sdk::initialize().expect("failed to init sdk");

let (mut width, mut height) = (0, 0);

sdk::get_interfaces().engine.get_screen_size(&mut width, &mut height);
```
