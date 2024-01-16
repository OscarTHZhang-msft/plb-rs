# Basic Setup

To enable Rust toolchain, go through the normal init process:

```ps1
> .\<WinFabRoot>\init.ps1 clear_cache
```

This will auto config cargo.exe to run in the current environment.

Then go to this directory (where the Cargo.toml located) and run:

```ps1
cargo build
```