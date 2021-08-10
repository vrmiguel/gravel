mod build_system;
mod command;
mod manifest;
mod query;
mod maven;

use query::query_current_dir;

fn main() {
    let build_systems = query_current_dir();

    // for token in xmlparser::Tokenizer::from("<tagname name='value'/>") {
    //     println!("{:?}", token);
    // }

    for build_system in build_systems {
        let exe = build_system.executable();
        let exe_found = which::which(exe).is_ok();
        let negate = if exe_found { "" } else { "not " };
        println!("{:?} [{}found]", build_system, negate);
    }
}
