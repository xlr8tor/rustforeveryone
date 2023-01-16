fn main() {
    if let Err(e) = word_count::get_args().and_then(word_count::run) {
        eprint!("{}", e);
        std::process::exit(1);
    }
}
