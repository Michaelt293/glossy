extern crate csv;
extern crate serde;
extern crate clap;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::error::Error;
use clap::ArgMatches;


#[derive(Deserialize)]
pub struct TermWithDescription {
    pub term: String,
    pub description: String,
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

fn search(term: &str, term_description: TermWithDescription) -> () {
    if term_description.term.to_lowercase() == term.to_lowercase() {
        println!("{}: {}",
                 term_description.term,
                 term_description.description)
    }
}

fn search_case_sensitive(term: &str, term_description: TermWithDescription) -> () {
    if term_description.term == term {
        println!("{}: {}",
                 term_description.term,
                 term_description.description)
    }
}

pub fn run(config: Config, term: &str) -> Result<(), Box<Error>> {
    let mut file_reader = csv::ReaderBuilder::new()
        .has_headers(config.file_has_headers)
        .from_path(&config.filepath)?;

    for record in file_reader.deserialize() {
        let term_description: TermWithDescription = record?;
        if config.case_sensitive {
            search_case_sensitive(term, term_description)
        } else {
            search(term, term_description)
        };
    }

    Ok(())
}
