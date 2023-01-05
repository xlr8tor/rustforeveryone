use clap::{App, Arg};
fn main() {
    let matches = App::new("Say Hello")
        .version("0.1.0")
        .author("Jeremiah Charles")
        .help("Input name to greet")
        .about("Say hello program")
        .arg(
            Arg::with_name("name")
                .short("n")
                .default_value("world")
                .takes_value(true)
                .min_values(1),
        )
        .get_matches();

    let text = matches.values_of_lossy("name").unwrap();

    println!("Hello, {}!", text.join(" "));
}
