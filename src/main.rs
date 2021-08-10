mod build_system;
use std::{convert::TryInto, env, path::PathBuf};

use build_system::BuildSystem;
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn main() {

    let check_for_build_system =
        |entry: DirEntry| -> Option<BuildSystem> { entry.path().try_into().ok() };

    let path = env::current_dir().expect("failed to obtain current directory!");

    let walker = WalkDir::new(path).into_iter();
    let build_system: Vec<BuildSystem> = walker
        .filter_entry(|e| !is_hidden(e))
        .filter_map(Result::ok)
        .filter_map(check_for_build_system)
        .collect();

    dbg!(build_system);
}
