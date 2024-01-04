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
    let _json = r#"
    {
        "article" : "How to work with json in rust",
        "author"  : "John Doe",
        "paragraphs":[{"name":"First"},{"name":"Second"}]
    }"#;

    let parsed: Article = read_json_typed(_json);
    println!("{:?}", parsed);
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed;
}
