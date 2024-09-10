use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed;
}

fn main() {
    let json = r#"
    {
        "article": "how to work with json in rust",
        "author": "Anuj",
        "paragraph": [
            {
                "name": "Starting sentence"
            },
            {
                "name": "Body of the paragraph"
            },
            {
                "name": "End of the paragraph"
            }
        ]
    }"#;

    let parsed: Article = read_json_typed(json);
    print!("{:?}", parsed.paragraph)
    // println!("\n\nThe name of the first paragraph is: {}", parsed.paragraph[0].name);
}