// Rust 2018 以降、クレートをインポートするための `extern crate` は不要
// https://doc.rust-jp.rs/edition-guide/rust-2018/path-changes.html#%E3%81%95%E3%82%88%E3%81%86%E3%81%AA%E3%82%89extern-crate
// extern crate minigrep;

use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
