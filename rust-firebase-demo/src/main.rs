use firebase_rs::*;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    response: String,
}

async fn add_user(firebase_client: &Firebase, user: &User) -> Response {
    let at_collection = firebase_client.at("users");

    let created_user = at_collection.set::<User>(&user).await;

    // println!("{:?}", created_user);
    // match created_user {
    //     Ok(_) => println!("Successfully added new user! {}", created_user.unwrap().data),
    //     Err(e) => eprintln!("Failed to add user: {}", e),
    // }

    return Response { response: created_user.unwrap().data };
}

#[tokio::main]
async fn main() {
    // demo struct for temp.
    let user = User {
        name: "Chirag".to_string(),
        email: "jani@test.com".to_string(),
    };

    // to connect with Firebase realtime database
    // Retrieve Firebase URL from environment variable
    let firebase_url = std::env
        ::var("FIRBASE_REALTIME_DATABASE_URL")
        .expect("Please set the FIRBASE_REALTIME_DATABASE_URL environment variable");
    // export FIRBASE_REALTIME_DATABASE_URL=<firebase_url> // enter this command in mac in terminal to set the env variable

    // Create Firebase instance using the retrieved URL
    let firebase = Firebase::new(&firebase_url).unwrap_or_else(|err|
        panic!("Failed to create Firebase instance: {}", err)
    );

    // creating user
    let res = add_user(&firebase, &user).await;
    println!("{:?}", res);
}

// convert a string to a response
#[allow(dead_code)]
fn string_to_response(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}

//convert a string to a user
#[allow(dead_code)]
fn string_to_user(s: &str) -> User {
    serde_json::from_str(s).unwrap()
}
