use std::sync::mpsc;
use std::{thread, time};

use loony::server::Server;
use loony::web::{self, middleware, App, HttpRequest};

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}

#[loony::main]
async fn run_app(tx: mpsc::Sender<Server>) -> std::io::Result<()> {
    // srv is server controller type, `dev::Server`
    let srv = web::server(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run();

    // send server controller to main thread
    let _ = tx.send(srv.clone());

    // run future
    srv.await
}

#[loony::main]
async fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=trace");
    env_logger::init();

    let (tx, rx) = mpsc::channel();

    println!("START SERVER");
    thread::spawn(move || {
        let _ = run_app(tx);
    });

    let srv = rx.recv().unwrap();

    println!("WATING 10 SECONDS");
    thread::sleep(time::Duration::from_secs(10));

    println!("STOPPING SERVER");
    // init stop server and wait until server gracefully exit
    srv.stop(true).await;
}
