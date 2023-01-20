use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
    http::StatusCode,
    Json,
};
use graphql::schema::{build_schema, AppSchema};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[cfg(debug_assertions)]
use dotenv::dotenv;
use serde_json::{json, Value};
use utils::auth::{decode_token, get_token_auth};

pub mod graphql;
pub mod prisma;
pub mod utils;

pub trait Bounds:Serialize + Sync + Send + Unpin + DeserializeOwned {}
impl<T> Bounds for T where T: Serialize + Sync + Send + Unpin + DeserializeOwned
{}

async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/api/graphql",
    )))
}

struct TokenResponse {
    access_token: String,
    refresh_token: String,
}

async fn users<T:Bounds>(Json(payload): Json<Value>) -> impl IntoResponse {

    let payload = payload;

    let jwt = payload.get("jwt").unwrap().as_str().unwrap();;

    let claims = decode_token(&jwt.to_string());

    let auth = get_token_auth(&jwt.to_string());

    match auth {
        Ok(_) => {
            let json = Json(serde_json::to_value(&claims).unwrap());
            return (StatusCode::OK, json);
        }
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            )
        }
    }
}


#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let schema = build_schema().await;

    let app = Router::new()
        .route(
            "/api/graphql",
            get(graphql_playground).post(graphql_handler),
        )
        .route("/user", post(users))
        .layer(Extension(schema));   

    println!("Playground: http://localhost:8080/api/graphql");

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}