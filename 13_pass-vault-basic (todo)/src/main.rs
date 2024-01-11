mod pentry;

use crate::pentry::prompt;
use crate::pentry::read_passwords_from_file;
use crate::pentry::ServiceInfo;

fn clr() {
    print!("{}[2J", 27 as char);
}

fn main() {
    clr();
    // let ascii =
    //     r#"
    //                 ▄▄▄▄▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄▄▄  ▄▄        ▄  ▄▄▄▄▄▄▄▄▄▄▄
    //                 ▐░░░░░░░░░░░▌▐░░░░░░░░░░░▌▐░░▌      ▐░▌▐░░░░░░░░░░░▌
    //                 ▀▀▀▀▀█░█▀▀▀ ▐░█▀▀▀▀▀▀▀█░▌▐░▌░▌     ▐░▌ ▀▀▀▀█░█▀▀▀▀
    //                     ▐░▌    ▐░▌       ▐░▌▐░▌▐░▌    ▐░▌     ▐░▌
    //                     ▐░▌    ▐░█▄▄▄▄▄▄▄█░▌▐░▌ ▐░▌   ▐░▌     ▐░▌
    //                     ▐░▌    ▐░░░░░░░░░░░▌▐░▌  ▐░▌  ▐░▌     ▐░▌
    //                     ▐░▌    ▐░█▀▀▀▀▀▀▀█░▌▐░▌   ▐░▌ ▐░▌     ▐░▌
    //                     ▐░▌    ▐░▌       ▐░▌▐░▌    ▐░▌▐░▌     ▐░▌
    //                 ▄▄▄▄▄█░▌    ▐░▌       ▐░▌▐░▌     ▐░▐░▌ ▄▄▄▄█░█▄▄▄▄
    //                 ▐░░░░░░░▌    ▐░▌       ▐░▌▐░▌      ▐░░▌▐░░░░░░░░░░░▌
    //                 ▀▀▀▀▀▀▀      ▀         ▀  ▀        ▀▀  ▀▀▀▀▀▀▀▀▀▀▀
    //     "#;

    // println!("{ascii}");

    loop {
        println!("1. Store Password");
        println!("2. List Entries");
        println!("3. Find Entry");
        println!("4. Quit");

        let mut choice = String::new();

        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clr();
                let service = ServiceInfo::new();
            }
            "2" => {
                clr();
                let serviecs = read_passwords_from_file().unwrap_or_else(|error| {
                    eprintln!("Error reading passwords: {}", error);
                    Vec::new()
                });

                for item in &services {
                    println!(
                        "
                            Service = {}
                            - Username = {}
                            - Password = {}
                        ",
                        item.service,
                        item.username,
                        item.password
                    );
                }
            }
            "3" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|error| {
                    eprintln!("Failed to load password file: {}", error);
                    Vec::new()
                });

                let search = prompt("Enter your search: ");
                for item in &services {
                    if item.service.as_str() == search.as_str() {
                        println(
                            "
                        Service = {}
                        - Username = {}
                        - Password = {}
                    ",
                            item.service,
                            item.username,
                            item.password
                        );
                    }
                }
            }
            "4" => {
                clr();
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid Choice, Please Try Again.");
            }
        }
    }
}

struct Password {
    user: String,
    password: String,
}
