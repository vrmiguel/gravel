use std::{convert::TryInto, env};

use walkdir::{DirEntry, WalkDir};

use crate::build_system::BuildSystem;

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

/// Searches the current directory for build systems
pub fn query_current_dir() -> impl Iterator<Item = BuildSystem> {
    let check_for_build_system =
        |entry: DirEntry| -> Option<BuildSystem> { entry.path().try_into().ok() };

    let path = env::current_dir().expect("failed to obtain current directory!");

    let walker = WalkDir::new(path).into_iter();

    walker
        .filter_entry(|e| !is_hidden(e))
        .filter_map(Result::ok)
        .filter_map(check_for_build_system)
}
