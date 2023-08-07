use std::process;

fn main() {
    if let Err(e) = wordclock::run() {
        println!("Error: {:?}", e);
        process::exit(1);
    }
}
