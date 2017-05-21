extern crate csv;
extern crate rustc_serialize;
extern crate clap;

use std::env;
use std::error::Error;
use clap::ArgMatches;


#[derive(RustcDecodable)]
pub struct TermWithMeaning {
    pub term: String,
    pub meaning: String,
}

pub struct Config {
    pub filepath: String,
    pub file_has_headers: bool,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(matches: &ArgMatches) -> Result<Config, &'static str> {
        match (env::var("GLOSSY_FILEPATH_HEADERS"), env::var("GLOSSY_FILEPATH_NO_HEADERS")) {
            (Ok(_), Ok(_)) => Err("GLOSSY_FILEPATH_HEADERS and GLOSSY_FILEPATH_NO_HEADERS both exported"),
            (Ok(f), Err(_)) => {
                Ok(Config {
                       filepath: f,
                       file_has_headers: true,
                       case_sensitive: matches.is_present("SENSITIVE"),
                   })
            }
            (Err(_), Ok(f)) => {
                Ok(Config {
                       filepath: f,
                       file_has_headers: false,
                       case_sensitive: matches.is_present("SENSITIVE"),
                   })
            }
            _ => Err("No environmental variable for glossy filepath found"),
        }
    }
}

fn search(term: &str, term_meaning: TermWithMeaning) -> () {
    if term_meaning.term.to_lowercase() == term.to_lowercase() {
        println!("{}: {}", term_meaning.term, term_meaning.meaning)
    }
}

fn search_case_sensitive(term: &str, term_meaning: TermWithMeaning) -> () {
    if term_meaning.term == term {
        println!("{}: {}", term_meaning.term, term_meaning.meaning)
    }
}

pub fn run(config: Config, term: &str) -> Result<(), Box<Error>> {
    let mut file_reader = csv::Reader::from_file(&config.filepath)
        .map(|f| f.has_headers(config.file_has_headers))?;

    for record in file_reader.decode() {
        let term_meaning: TermWithMeaning = record?;
        if config.case_sensitive {
            search_case_sensitive(term, term_meaning)
        } else {
            search(term, term_meaning)
        };
    }

    Ok(())
}
