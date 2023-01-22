use serde::{Serialize, Deserialize};
use jsonwebtoken::{errors::Error, encode, Header, EncodingKey, decode, DecodingKey, Validation, TokenData};
use chrono;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
   pub sub: i32,
    pub exp: usize,
}

pub fn is_auth(jwt: String) -> bool {
   let token = decode::<Claims>(&jwt, &DecodingKey::from_secret("secret".as_ref()), &Validation::default());
   match token {
    Ok(_) => true,
    Err(_) => false,
   }
}

pub fn get_token_auth(jwt: &String) -> Result<TokenData<Claims>, Error> {
    return decode::<Claims>(&jwt, &DecodingKey::from_secret("secret".as_ref()), &Validation::default());
}

pub fn decode_token(jwt: String) -> Claims {
   let token = decode::<Claims>(&jwt, &DecodingKey::from_secret("secret".as_ref()), &Validation::default());
   
   let empty_claims: Claims = Claims { sub: 0, exp: 0 };

   match token {
    Ok(token)=> token.claims,
    Err(_error) => empty_claims,
   }
}

pub fn create_access_token(user_id: i32) -> String {

    let claims = Claims {
        sub: user_id,
        exp: (chrono::prelude::Utc::now().timestamp() + 3600) as usize,
    };

    return encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();
}

pub fn create_refresh_token(user_id: i32) -> String {

    let claims = Claims {
        sub: user_id,
        exp: (chrono::prelude::Utc::now().timestamp() + 36000) as usize,
    };

    return encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();
}