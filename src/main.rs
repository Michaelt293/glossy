extern crate glossy;
extern crate csv;
extern crate rustc_serialize;
extern crate clap;

use glossy::*;
use std::process;
use std::io::prelude::*;
use clap::{Arg, App};


fn main() {
    let matches = App::new("Glossy")
        .version("0.1.0")
        .author("Michael Thomas <Michaelt293@gmail.com>")
        .about("Find descriptions for acronyms etc.")
        .arg(Arg::with_name("SENSITIVE")
                 .short("s")
                 .help("Sets search to case sensitive"))
        .arg(Arg::with_name("TERM")
                 .help("Term (e.g., acronym) to search for")
                 .required(true)
                 .index(1))
        .get_matches();

    let mut stderr = std::io::stderr();

    let config = Config::new(&matches).unwrap_or_else(|err| {
                                                          writeln!(&mut stderr,
                                                          "Problem with environmental variable: {}",
                                                          err)
                                                         .expect("Could not write to stderr");
                                                          process::exit(1);
                                                      });

    let term = matches.value_of("TERM").unwrap();

    if let Err(e) = glossy::run(config, term) {
        writeln!(&mut stderr, "Application error: {}", e).expect("Could not write to stderr");
        process::exit(1);
    }
}
