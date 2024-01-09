use actix_web::{ get, web, App, HttpResponse, HttpServer, Responder };

#[get("/test")]
async fn test() -> impl Responder {
    HttpResponse::Ok().body("Hellow from Test!!! :)")
}

#[get("/path")]
async fn path() -> impl Responder {
    HttpResponse::Ok().body("Hey there from path!")
}

async fn index() -> impl Responder {
    "Hello World"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(test)
            .service(path)
            .service(web::scope("/app").route("/index.html", web::get().to(index)))
    })
        .bind(("127.0.0.1", 8080))?
        .run().await?;

    Ok(())
}
