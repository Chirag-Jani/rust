use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filepath: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("Invalid length");
        };

        let query = args[1].clone();
        let filepath = args[2].clone();

        Ok(Self { query, filepath })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string(config.filepath)?; // ? does the error handling and returns whatever error might occur

    // println!("\n\nFinding {:?} in the below", config.query);
    // println!("File Content:\n\n{data}");

    let split_data: Vec<_> = data.split(' ').collect();
    println!("\n\n\n{:?}\n\n\n", split_data);

    for word in split_data {
        if word == config.query {
            println!("Word found!!!");
            return Ok(());
        }
    }

    println!("Word not found!!!");

    Ok(())
}
