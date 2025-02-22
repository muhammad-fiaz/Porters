use colored::*;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("{}", "Usage: porters <command>".red());
        process::exit(1);
    }

    match args[1].as_str() {
        "init" => init::run(),
        "new" => {
            if args.len() < 3 {
                eprintln!("{}", "Usage: porters new <projectname>".red());
                process::exit(1);
            }
            init::create_new_project(&args[2]);
        }
        "build" => build_system::run(),
        _ => {
            eprintln!("{}", "Unknown command".red());
            process::exit(1);
        }
    }
}
