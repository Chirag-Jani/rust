use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraphs: Vec<Paragraph>,
}

fn main() {
    // let _json = r#"
    // {
    //     "article" : "How to work with json in rust",
    //     "author"  : "John Doe",
    //     "paragraphs":[{"name":"First"},{"name":"Second"}]
    // }"#;

    // typical way to create JSON and it can be accessed like normal one
    let _json_twp: Article = Article {
        article: String::from("this is the second artical"),
        author: String::from("jani"),
        paragraphs: vec![
            Paragraph {
                name: String::from("First"),
            },
            Paragraph {
                name: String::from("Second"),
            },
        ],
    };

    // let parsed: Article = read_json_typed(_json);
    // println!("{:?}", parsed);

    let readable_json = serde_json::to_string(&_json_twp).unwrap(); // this will convert in to string
    println!("{:?}", readable_json);
}

// fn read_json_typed(raw_json: &str) -> Article {
//     let parsed: Article = serde_json::from_str(raw_json).unwrap();
//     return parsed;
// }
