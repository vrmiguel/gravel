use std::{convert::TryInto, env};

use walkdir::{DirEntry, WalkDir};

use crate::{build_system::BuildSystem, manifest::Manifest};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

/// Searches the current directory for build systems
pub fn query_current_dir() -> impl Iterator<Item = Manifest> {
    let check_for_build_system = |entry: DirEntry| -> Option<Manifest> {
        let build_system: BuildSystem = entry.path().try_into().ok()?;
        let path = entry.into_path();

        let manifest = Manifest::new(path, build_system);

        Some(manifest)
    };

    let path = env::current_dir().expect("failed to obtain current directory!");

    WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(Result::ok)
        .filter_map(check_for_build_system)
}
