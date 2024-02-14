use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let config = Config::build(&args);

    match config {
        Ok(config_instance) => {
            if let Err(e) = minigrep::run(config_instance) {
                eprintln!("Application error: {e}");
                process::exit(1);
            }
        }
        Err(e) => eprintln!("Problem parsing arguments: {e}"),
    }

    // the below is the shorthand and works better
    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     println!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });
}
