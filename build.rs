use std::path::PathBuf;

fn main() {
    // If CARGO_LOCK is set, just pass it to get_cargo_lock_path function
    if let Ok(cargo_lock_path) = std::env::var("CARGO_LOCK") {
        // Check if cargo_lock_path is absolute
        if !PathBuf::from(&cargo_lock_path).is_absolute() {
            panic!("CARGO_LOCK={} must be an absolute path", cargo_lock_path);
        }
        println!("cargo:rustc-env=CARGO_LOCK_PATH={cargo_lock_path}");
        return;
    }
    // Otherwise check if the crate is in workspace
    let cargo_home = PathBuf::from(std::env::var("CARGO_HOME").unwrap());
    let cargo_manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    if cargo_manifest_dir.starts_with(cargo_home) {
        panic!("\n\
        The crate `zenoh-ffi-workspace` located at {} is being used as a cargo dependency.\n\
        Being not located inside the workspace it cannot determine the path to workspace's `Cargo.lock`.\n\
        \n\
        Two solutions are possible:\n\
        1. Explicitly pass the path to Cargo.lock in the environment variable CARGO_LOCK. E.g.\n\
        \n\
        CARGO_LOCK=$PWD/Cargo.lock cargo build\n\
        \n\
        2. Add the clone of `zenoh-ffi-workspace` project to the workspace\n\
           and add \"patch\" section to `Cargo.toml` to make other crates to use this local version\n\
           In this case the `cargo build` command will work without additional configuration\n\
           \n\
           git clone https://github.com/eclipse-zenoh/zenoh-ffi-workspace\n\
           \n\
           [workspace]\n\
           members = [\n\
               \"zenoh-ffi-workspace\",...\n\
           ]\n\
           [patch.crates-io]\n\
           \"zenoh-ffi-workspace\" = {{ path = \"zenoh-ffi-workspace\" }}\n\
           [patch.'https://github.com/eclipse-zenoh/zenoh-ffi-workspace']\n\
           \"zenoh-ffi-workspace\" = {{ path = \"zenoh-ffi-workspace\" }}\n\
           \n
        ", cargo_manifest_dir.display());
    }
    let workspace_root = project_root::get_project_root().unwrap();
    let cargo_lock_path = workspace_root.join("Cargo.lock");
    println!("cargo:rustc-env=CARGO_LOCK_PATH={}", cargo_lock_path.display());
}
