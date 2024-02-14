use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filepath: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("Invalid length");
        };

        let query = args[1].clone();
        let filepath: String = args[2].clone();
        let ignore_case;

        if args[3] == "true" {
            ignore_case = true;
        } else {
            ignore_case = false;
        }

        Ok(Self {
            query,
            filepath,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? does the error handling and returns whatever error might occur
    let data = fs::read_to_string(config.filepath)?;

    if config.ignore_case == true {
        for line in search_insensitive(&config.query, &data) {
            println!("{line}");
        }
    } else {
        for line in search(&config.query, &data) {
            println!("{line}");
        }
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

pub fn search_insensitive<'lifetime>(query: &str, contents: &'lifetime str) -> Vec<&'lifetime str> {
    // convert query to lowercase

    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        // don't convert the whole content to lowercase but just the line you are on while iterating and compare
        if line.to_lowercase().contains(&query) {
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
    #[ignore]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "e wilL";
        let contents = "This is the sentence in which
the code wiLl try to 
find query.";

        let s = search(query, contents);

        println!("Found data: {:?}", s);
    }

    #[test]
    fn case_insensitive() {
        let query = "e wilL";
        let contents = "This is the sentence in which
the code wiLl try to 
find query.";

        let s = search_insensitive(query, contents);

        println!("Found data: {:?}", s);
    }
}
