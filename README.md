# zenoh-ffi-workspace

A technical crate providing the path to the workspace's Cargo.lock, which is necessary to correctly build the zenoh-ffi crate.

## Usage

This crate allows zenoh-ffi to determine the path to the workspace's `Cargo.lock` file. zenoh-ffi needs it to correctly
determine the size and alignment of so-called "opaque types": blind repr(C) structures equivalent to corresponding Rust ones.
Unfortunately, cargo doesn't allow dependent crates to know the path to the workspace where they're being built,
so additional tweaks are necessary to pass this information to the zenoh-ffi crate.

### Solution 1: Environment Variable

Set the `CARGO_LOCK` environment variable to the absolute path of your `Cargo.lock`:

```bash
CARGO_LOCK=$PWD/Cargo.lock cargo build
```

zenoh-ffi-workspace simply passes this value to the zenoh-ffi crate.

### Solution 2: Workspace Integration

Add the crate to your workspace and substitute the original "zenoh-ffi-workspace" with a local version:

1. Clone the repository:

```bash
git clone https://github.com/eclipse-zenoh/zenoh-ffi-workspace
```

2. Add to your `Cargo.toml`:

```toml
[workspace]
members = [
    "zenoh-ffi-workspace",
    # ... other members
]

[patch.crates-io]
"zenoh-ffi-workspace" = { path = "zenoh-ffi-workspace" }

[patch.'https://github.com/eclipse-zenoh/zenoh-ffi-workspace']
"zenoh-ffi-workspace" = { path = "zenoh-ffi-workspace" }
```

With this setup, the zenoh-ffi-workspace project can automatically pass the Cargo.lock
path to zenoh-ffi, and `cargo build` works without additional configuration.
