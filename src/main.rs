use std::{error::Error, process, thread::sleep, time};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Single reading of the clock
    #[arg(short, long)]
    single: bool,

    /// Continuous reading of the clock
    #[arg(short, long)]
    continuous: bool,

    /// Display debug information: time words and exact time
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    if args.continuous && args.single {
        println!("Error: Cannot use '--single' and '--continuous' simultanuously.");
        process::exit(2);
    }

    if args.continuous {
        if let Err(e) = continuous_mode(args.debug) {
            println!("Error: {:?}", e);
            process::exit(1);
        }
    } else if args.single {
        if let Err(e) = wordclock::run(args.debug) {
            println!("Error: {:?}", e);
            process::exit(1);
        }
    }
}

fn continuous_mode(exact: bool) -> Result<(), Box<dyn Error>> {
    loop {
        let _ = wordclock::run(exact)?;

        let one_second = time::Duration::from_secs(1);
        sleep(one_second);
    }
}
