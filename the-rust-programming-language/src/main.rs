use std::{env, process::exit};

use crate::chapter_12::lib::{Config, run};

mod chapter_12;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        exit(1);
    }
}
