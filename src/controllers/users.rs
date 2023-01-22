
use actix_web::{HttpRequest, HttpResponse, Error, cookie::{CookieJar, Cookie}, error::ErrorUnauthorized};
use serde::{Deserialize, Serialize};

use crate::utils::auth::{decode_token, create_refresh_token, create_access_token, get_token_auth};


#[derive(Deserialize, Serialize)]
pub struct Response {
    user_id:i32,
    access_token:String,
}

#[derive(Deserialize, Serialize)]
pub struct Request {
    jwt: String,
}


pub async fn users(req: HttpRequest) -> Result<HttpResponse, Error>{

    let authorization = req.cookie("session_id").unwrap_or(Cookie::new("session_id", "unauth")).value().to_string();

    let auth = get_token_auth(&authorization);

    let claims = decode_token(authorization);

    let response = Response {
        user_id: claims.sub,
        access_token: create_access_token(claims.sub),
    };

    let mut jar =  CookieJar::new();

    jar.add(Cookie::new("session_id", create_refresh_token(claims.sub)));

    let cookie = jar.get("session_id").unwrap().clone();    

    let json_response = serde_json::to_string(&response).unwrap();

    match auth {
        Ok(_) =>     Ok(HttpResponse::Ok()
                    .cookie(cookie)
                    .body(json_response)),
        Err(err) =>    Err(ErrorUnauthorized(err))
    }
}