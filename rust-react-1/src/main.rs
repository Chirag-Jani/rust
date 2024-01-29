use actix_web::{ get, web, HttpResponse, HttpServer, App, Responder };
use serde::Serialize;
mod queryParams;
use queryParams::{ ImageParams, generate_image };

#[derive(Serialize)]
struct ApiData {
    success: bool,
    data: Vec<u8>,
}

#[get("/get-image")]
async fn get_image(query: web::Query<ImageParams>) -> impl Responder {
    let queryData = &query.into_inner();
    let image_response = generate_image(queryData);

    let res = ApiData {
        success: true,
        data: image_response,
    };

    HttpResponse::Ok().json(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| { App::new().service(get_image) })
        .bind(("127.0.0.1", 8080))?
        .run().await?;

    Ok(())
}
