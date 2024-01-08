use std::collections::HashMap;
use clap::Parser;

fn main() {
    let args = Cli::parse();
    let mut todo = Todo::new().expect("Error initializing todo list");

    if args.action == "add" {
        // adding todo to the hashmap (in short updating the hashmap)
        todo.add_todo(args.item);

        // calling save function to update into the file
        match todo.save() {
            // we can match this because we are gettign Ok in return
            Ok(_) => println!("Item added!"),
            Err(e) => eprintln!("Failed to add item: {}", e),
        }
    } else if args.action == "complete" {
        // mark completed
        match todo.complete(&args.item) {
            None => println!("'{}' is not present in the list", args.item),
            Some(_) =>
                // calling save function to update into the file
                match todo.save() {
                    Ok(_) => println!("todo saved"),
                    Err(why) => println!("An error occurred: {}", why),
                }
        }
    }
}

#[derive(Parser)]
struct Cli {
    action: String,
    item: String,
}

#[derive(Debug)]
struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    // creates instance or somethin', i don't understand this func
    // I understand that, db.json file is being created (using serde_json)
    fn new() -> Result<Todo, std::io::Error> {
        let f = std::fs::OpenOptions::new().write(true).create(true).read(true).open("db.json")?;

        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() =>
                Ok(Todo {
                    map: HashMap::new(),
                }),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }

    // function to add todo
    fn add_todo(&mut self, key: String) {
        println!("Todo item is: {key}");

        // adding item to the hashmap
        self.map.insert(key, false);
    }

    // this function implements the changes in the db.json file once function to add todo or remove todo or any other
    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        // i guess that this is opening the file to update
        let save_data = std::fs::OpenOptions
            ::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("db.json")?;

        // this is updating the file by passing self.map
        serde_json::to_writer_pretty(save_data, &self.map);

        // printing all the items on the console
        for (index, (key, value)) in self.map.iter().enumerate() {
            println!("{}. {} : {}", index + 1, key, value);
        }

        // returning OK
        Ok(())
    }

    // marking complete
    // getting key in the args
    fn complete(&mut self, key: &String) -> Option<()> {
        // matching item and marking true
        match self.map.get_mut(key) {
            // finds value and marks true
            Some(val) =>
                Some({
                    *val = true;
                }),
            // returns none if doesn't find any value
            None => None,
        }
    }
}
