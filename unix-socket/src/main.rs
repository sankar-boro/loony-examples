use loony::web::{self, middleware, App, HttpRequest};

async fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

#[loony::main]
#[cfg(unix)]
async fn main() -> std::io::Result<()> {
    ::std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    web::server(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .service((
                web::resource("/index.html")
                    .route(web::get().to(|| async { "Hello world!" })),
                web::resource("/").to(index),
            ))
    })
    .bind_uds("/tmp/actix-uds.socket")?
    .run()
    .await
}

#[cfg(not(unix))]
fn main() -> std::io::Result<()> {
    println!("not supported");
    Ok(())
}
