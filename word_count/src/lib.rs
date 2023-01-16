use clap::{App, Arg};
use std::{error::Error, io::BufRead};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub struct Config {
    file: Vec<String>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("Emulate wc (word count)")
        .version("0.1.0")
        .author("Jeremiah Charles")
        .help("Input file(s)")
        .about("rust word count")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .takes_value(true)
                .multiple(true),
            
        )
        .get_matches();
    Ok(Config {
        file: matches.values_of_lossy("file").unwrap(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for 
}

fn open(filename: &str) -> MyResult<Box <dyn BufRead>> {}