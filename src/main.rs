mod build_system;
use std::{convert::TryInto, path::PathBuf};

use build_system::BuildSystem;

fn main() {
    let path = PathBuf::from("Cargo.toml");
    let path = &path;
    let system: BuildSystem = path.try_into().unwrap();
}
