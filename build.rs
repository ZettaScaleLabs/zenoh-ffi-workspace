use std::path::Path;

fn main() {
    let cargo_home = std::env::var("CARGO_HOME").unwrap();
    let cargo_home = Path::new(&cargo_home);
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_manifest_dir = Path::new(&cargo_manifest_dir);
    if cargo_manifest_dir.starts_with(cargo_home) {
        panic!("This module should not be used as a cargo dependency, it's purpose is to provide access to workspace directory to zenoh-ffi/build.rs\n\
        Place the zenoh-ffi-workspace source to the workspace and use [patch.crates.io] and [patch.\"https://github.com/eclipse-zenoh/zenoh-ffi\"] sections\n\
        to make zenoh-ffi use local version of zenoh-ffi-workspace.\n\
        See README.md and crate documentation for more information.
        ");
    }
}
