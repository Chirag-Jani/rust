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
    // ? does the error handling and returns whatever error might occur
    let data = fs::read_to_string(config.filepath)?;

    for line in search(&config.query, &data) {
        println!("{line}");
    }

    // println!("\n\nFinding {:?} in the below", config.query);
    // println!("File Content:\n\n{data}");

    // BELOW MY CODE - NOT FROM THE BOOK
    // let split_data: Vec<_> = data.split(' ').collect();
    // println!("\n\n\n{:?}\n\n\n", split_data);

    // for word in split_data {
    //     if word == config.query {
    //         println!("Word found!!!");
    //         return Ok(());
    //     }
    // }

    // println!("Word not found!!!");

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            // do something with line
            results.push(line);
            println!("Found in line: {line}")
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
