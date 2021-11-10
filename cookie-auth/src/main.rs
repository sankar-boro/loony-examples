use loony::web::{self, middleware, App, HttpResponse};
use loony_identity::{CookieIdentityPolicy, Identity, IdentityService};

async fn index(id: Identity) -> String {
    format!(
        "Hello {}",
        id.identity().unwrap_or_else(|| "Anonymous".to_owned())
    )
}

async fn login(id: Identity) -> HttpResponse {
    id.remember("user1".to_owned());
    HttpResponse::Found().header("location", "/").finish()
}

async fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Found().header("location", "/").finish()
}

#[loony::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    web::server(|| {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-example")
                    .secure(false),
            ))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .service((
                web::resource("/login").route(web::post().to(login)),
                web::resource("/logout").to(logout),
                web::resource("/").route(web::get().to(index)),
            ))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
