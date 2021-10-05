mod commands;

use seahorse::{App};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new("Pledger")
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("pledger [args]")
        .command(commands::check::create());
    
    app.run(args);
}