use firebase_rs::*;
use serde::{ Serialize, Deserialize };
use actix_web::{ get, post, web, App, HttpResponse, HttpServer, Responder };
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    response: String,
}

// helper to create user
async fn add_user(firebase_client: &Firebase, user: &User) -> Response {
    let at_collection = firebase_client.at("users");

    let created_user = at_collection.set::<User>(&user).await;

    return Response { response: created_user.unwrap().data };
}

fn get_firebase_instance() -> Firebase {
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

    return firebase;
}

#[get("/get-user")]
async fn get_user(name: web::Form<HashMap<String, String>>) -> HttpResponse {
    let fb = get_firebase_instance();
    let data = fb.at("users").at(&name.0["name"]);
    let user = data.get::<User>().await;
    println!("{:?}", user);

    match data.get::<User>().await {
        Ok(user) => {
            // Data was found
            println!("Got the user: {:?}", user);
            HttpResponse::Ok().body(format!("Got the name: {:?}", name.0["name"]))
        }

        Err(err) => {
            // Log other errors for further investigation
            println!("Firebase Error: {:?}", err);
            // Return an appropriate response, e.g., Internal Server Error
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/get-all")]
async fn get_all_users() -> impl Responder {
    let fb = get_firebase_instance();
    let firebase = fb.at("users");
    let users_result: Result<HashMap<String, User>, _> = firebase.get().await;

    match users_result {
        Ok(users) => {
            // Successfully retrieved users, respond with JSON
            HttpResponse::Ok()
                .content_type("application/json")
                .body(serde_json::to_string(&users).unwrap())
        }
        Err(err) => {
            // Handle the error, for example, log it and respond with an error status
            println!("Error retrieving users: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/add-user")]
async fn create_user(user: web::Form<User>) -> impl Responder {
    let fb = get_firebase_instance();

    // creating user
    let _res = add_user(&fb, &user).await;
    // println!("{:?}", res);

    // Handle the user data as needed
    let response_json = serde_json::to_string(&user).unwrap();

    HttpResponse::Ok().content_type("application/json").body(response_json)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new().service(get_all_users).service(create_user).service(get_user)
    })
        .bind(("127.0.0.1", 8080))?
        .run().await?;

    Ok(())
}
