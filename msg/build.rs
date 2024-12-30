use std::{env, path::Path};
fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let airsim_path = Path::new(&project_dir).join("airsim");
    println!("cargo:rustc-env=ROSRUST_MSG_PATH={}", project_dir);
    println!("cargo:rerun-if-changed={}", airsim_path.display());
}
