use std::path::Path;

fn main() {
    let cargo_home = std::env::var("CARGO_HOME").unwrap();
    let cargo_home = Path::new(&cargo_home);
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_manifest_dir = Path::new(&cargo_manifest_dir);
    if cargo_manifest_dir.starts_with(cargo_home) {
        panic!("\n\
        The module `zenoh-ffi-workspace` should not be used as a cargo dependency.\n\
        One of its purposes is to provide access to the workspace directory for zenoh-ffi/build.rs.\n\
        To enable this, zenoh-ffi-workspace/build.rs must be located in the workspace directory.\n\n\
        Place the zenoh-ffi-workspace source in the workspace (with git clone or git submodule)\n\
        and use [patch.crates.io] and [patch.\"https://github.com/eclipse-zenoh/zenoh-ffi\"] sections\n\
        to make zenoh-ffi use this local version of zenoh-ffi-workspace.\n\
        See README.md and crate documentation for more information.
        ");
    }

    let workspace_root = project_root::get_project_root().unwrap();
    let cargo_lock = workspace_root.join("Cargo.lock");
    let target = std::env::var("TARGET").unwrap();
    println!("cargo:rustc-env=CARGO_LOCK={}", cargo_lock.display());
    println!("cargo:rustc-env=TARGET={}", target);
}
