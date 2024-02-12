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
    fn summarize(&self) -> &String {
        &self.username
    }
}

pub trait Summary {
    fn summarize(&self) -> &String;
}

fn get_summary(twt: &impl Summary) {
    println!("{:}", twt.summarize());
}

fn main() {
    let t1 = Tweet::new(
        "chiragjani001".to_owned(),
        "this is the content of tweet".to_owned(),
        12345,
    );

    get_summary(&t1);
}
