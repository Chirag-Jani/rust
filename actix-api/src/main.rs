use actix_web::{ get, web, App, HttpResponse, HttpServer, Responder };

struct AppliactionState {
    app_name: String,
}

struct AnotherState {
    is_another: bool,
    built_who: String,
}

#[get("/path")]
async fn path() -> impl Responder {
    HttpResponse::Ok().body("Hey there from path!")
}

async fn index(
    data: web::Data<AppliactionState>,
    data2: web::Data<AnotherState>
) -> impl Responder {
    let app_name = &data.app_name;
    let creator = &data2.built_who;
    let state = &data2.is_another;
    format!("Hello from {app_name} which is build by {creator}, true?? {state}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(
                web::Data::new(AppliactionState {
                    app_name: String::from("Test app"),
                })
            )
            .app_data(
                web::Data::new(AnotherState {
                    is_another: true,
                    built_who: String::from("Jani"),
                })
            )
            .service(path)
            .service(web::scope("/app").route("/index.html", web::get().to(index)))
    })
        .bind(("127.0.0.1", 8080))?
        .run().await?;

    Ok(())
}
