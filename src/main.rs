use std::env;
use std::process;

use minigrep::Config;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    let config_var = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    // println!("Searching: '{}', in file: {}", config_var.query,config_var.filepath);

    match minigrep::run(config_var) {
        Err(e) => {
            eprintln!("Application error: {e}");
            process::exit(1);
        }
        Ok(_) => (),
    }

    
}

