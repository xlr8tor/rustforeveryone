use clap::{App, Arg, ArgMatches};
use std::fs;
use std::path::Path;

fn get_args() -> ArgMatches<'static> {
    App::new("Howler (upper-cases input)")
        .version("0.1.0")
        .author("Jeremiah Charles")
        .about("Rust howler program")
        .help("Input string or file")
        .arg(Arg::with_name("text").required(true))
        .arg(Arg::with_name("output").short("o").takes_value(true))
        .get_matches()
}
fn main() {
    let matches = get_args();
    let text = matches.value_of("text").unwrap();
    let content = if Path::new(text).exists() {
        fs::read_to_string(text).expect("should exist")
    } else {
        text.to_string()
    };

    println!("{}", content.to_uppercase())
}
