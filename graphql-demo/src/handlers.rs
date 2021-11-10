use std::sync::Arc;

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use loony::web::{self, Error, HttpResponse};

use crate::db::Pool;
use crate::schemas::root::{create_schema, Context, Schema};

pub async fn graphql(
    pool: web::types::Data<Pool>,
    schema: web::types::Data<Arc<Schema>>,
    data: web::types::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = Context {
        dbpool: pool.get_ref().to_owned(),
    };
    let res = web::block(move || {
        let res = data.execute(&schema, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res))
}

pub async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql_source("/graphql"))
}

pub fn register(config: &mut web::ServiceConfig) {
    let schema = std::sync::Arc::new(create_schema());
    config
        .data(schema)
        .route("/graphql", web::post().to(graphql))
        .route("/graphiql", web::get().to(graphql_playground));
}
