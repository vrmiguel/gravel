mod build_system;
mod query;

use query::query_current_dir;

fn main() {
    let build_systems: Vec<_> = query_current_dir().collect();

    for build_system in build_systems {
        let exe = build_system.executable();
        let _ = dbg!(which::which(exe));
    }

    dbg!(query_current_dir().collect::<Vec<_>>());
}