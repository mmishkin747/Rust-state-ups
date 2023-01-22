fn main() {
    if let Err(e) = rups::get_args().and_then(rups::run) {
        eprintln!("{}", e);
        std::process::exit(10);
    }
}