

fn main() {
    if let Err(err) = wordcount::run() {
        eprint!("Error: {:?}", err);
        std::process::exit(1);
    }
}
