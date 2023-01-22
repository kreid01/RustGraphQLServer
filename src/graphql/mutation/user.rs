use async_graphql::{Context, InputObject, Object, Result};

use bcrypt::{DEFAULT_COST, hash, verify};

use crate::{
    graphql::types::User,
    prisma::{PrismaClient, user},
    utils::auth::{create_access_token, create_refresh_token}
};


#[derive(InputObject)]
pub struct CreateUserInput {
    pub display_name: String,
    pub email: String,
    pub password: String,
}

#[derive(InputObject)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

pub struct LoginResponse {
    pub user_id: i32,
    pub access_token: String,
}


#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn  register(&self, ctx: &Context<'_>, input: CreateUserInput) -> Result<User> {
        let db = ctx.data::<PrismaClient>().unwrap();

        let hashed = hash(input.password.to_string(), DEFAULT_COST)?;
        
        let created = db
            .user()
            .create(input.display_name, input.email, hashed, vec![])
            .exec()
            .await?;

        Ok(created.into())
    }

    pub async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<String> {
        let db = ctx.data::<PrismaClient>().unwrap();
 

        let user =  db
            .user()
            .find_first(vec![user::email::equals(input.email)])
            .exec()
            .await
            .unwrap();

        let valid = verify(input.password.to_string(), &user.as_ref().unwrap().password.to_string());

        let response = LoginResponse {
            user_id: user.as_ref().unwrap().id,
            access_token: create_access_token(user.unwrap().id.clone())
       };       

       let empty_response = LoginResponse {
            user_id: 0,
            access_token: "".to_string()
       };

        ctx.insert_http_header("set-cookie", create_refresh_token(response.user_id));
        
       match valid {
           Ok(_) => Ok(response.access_token),
           Err(_) => Ok(empty_response.access_token),
       }
    }
}