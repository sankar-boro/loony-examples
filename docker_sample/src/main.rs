use loony::web::{self, App, HttpResponse};

#[web::get("/")]
async fn index() -> HttpResponse {
    println!("GET: /");
    HttpResponse::Ok().body("Hello world!")
}

#[web::get("/again")]
async fn again() -> HttpResponse {
    println!("GET: /again");
    HttpResponse::Ok().body("Hello world again!")
}

#[loony::main]
async fn main() -> std::io::Result<()> {
    println!("Starting loony-web server");

    web::server(|| App::new().service((index, again)))
        .bind("0.0.0.0:5000")?
        .run()
        .await
}
