mod build_system;
mod query;

use query::query_current_dir;

fn main() {
    dbg!(query_current_dir().collect::<Vec<_>>());
}
