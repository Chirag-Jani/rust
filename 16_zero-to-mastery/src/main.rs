use std::collections::HashMap;

fn main() {
    let mut members: HashMap<String, String> = HashMap::new();
    members.insert("Jani".to_owned(), "INtern".to_owned());
    members.insert("CHirag Limb".to_owned(), "TL".to_owned());
    println!("The final list of adults is: {:?}", members);
}
