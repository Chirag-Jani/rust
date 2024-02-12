use std::iter::Sum;

#[derive(Debug)]
#[allow(dead_code)]
struct Tweet {
    pub username: String,
    pub content: String,
    pub timestamp: u64,
}

impl Tweet {
    fn new(username: String, content: String, timestamp: u64) -> Self {
        Self {
            username,
            content,
            timestamp,
        }
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        self.username.to_owned()
    }
}

pub trait Summary {
    fn summarize(&self) -> String {
        // the below is the default implementation
        String::from("Read more...")
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Artical {
    content: String,
}

impl Summary for Artical {}

fn get_summary(twt: &impl Summary) {
    println!("{:}", twt.summarize());
}

fn main() {
    let t1 = Tweet::new(
        "chiragjani001".to_owned(),
        "this is the content of tweet".to_owned(),
        12345,
    );

    let a1 = Artical {
        content: "Ehh.. Whateverrr!!".to_owned(),
    };

    get_summary(&a1);
    get_summary(&t1);
}
