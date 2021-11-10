//! Actix web juniper example
//!
//! A simple example integrating juniper in actix-web
use std::io;

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use loony::web::{self, middleware, App, Error, HttpResponse};

mod schema;

use crate::schema::{create_schema, Schema};

#[web::get("/graphiql")]
async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[web::post("/graphql")]
async fn graphql(
    st: web::types::Data<Schema>,
    data: web::types::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

#[loony::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Create Juniper schema
    let schema = web::types::Data::new(create_schema());

    // Start http server
    web::server(move || {
        App::new()
            .app_data(schema.clone())
            .wrap(middleware::Logger::default())
            .service((graphql, graphiql))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
