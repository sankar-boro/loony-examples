use loony::web::{self, middleware, App};

use async_ex2::appconfig::config_app;

#[loony::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "loony=info");
    env_logger::init();

    web::server(|| {
        App::new()
            .configure(config_app)
            .wrap(middleware::Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
