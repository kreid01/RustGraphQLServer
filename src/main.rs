use async_graphql::{http::{GraphiQLSource}};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use graphql::schema::{build_schema, AppSchema};
use actix_web::{guard, web::{self},
    web::{Data, post}, App, HttpResponse, HttpServer, 
    Result};

pub mod graphql;
pub mod prisma;
pub mod utils;
pub mod controllers;

use crate::controllers::users::users;

#[cfg(debug_assertions)]

async fn graphql_handler(schema: web::Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> Result<HttpResponse>{
    Ok(HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(GraphiQLSource::build().endpoint("/").finish()))
}




#[tokio::main]
async fn main() -> std::io::Result<()>  {
    let schema = build_schema().await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(graphql_handler))
            .service(web::resource("/").guard(guard::Get()).to(graphql_playground))
            .route("user", post().to(users))
        }).bind(&"0.0.0.0:8080")?
        .run()
        .await
}