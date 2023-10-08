use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

#[utoipa::path(
    context_path = "",
    responses(
        (status = 200, description = "Hello from api 1", body = String)
    )
)]
#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// fn main() {
//     println!("Hello, world!");
// }


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    #[derive(OpenApi)]
    #[openapi(paths(hello))]
    struct ApiDoc1;

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![
                (
                    Url::new("api1", "/api-docs/openapi1.json"),
                    ApiDoc1::openapi(),
                ),
            ]))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}