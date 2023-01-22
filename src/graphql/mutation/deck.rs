use async_graphql::{Context, InputObject, Object, Result};

use crate::{
    graphql::types::Deck,
    prisma::{user, PrismaClient},
};

#[derive(InputObject)]
pub struct CreateDeckInput {
    pub title: String,
    pub user_id: i32,
}

#[derive(Default)]
pub struct DeckMutation;

#[Object]
impl DeckMutation {
    pub async fn create_deck(&self, ctx: &Context<'_>, input: CreateDeckInput) -> Result<Deck> {
        let db = ctx.data::<PrismaClient>().unwrap();

        let created = db
            .deck()
            .create(
                user::id::equals(input.user_id),
                input.title.to_string(),
                vec![]           
            )
            .exec()
            .await?;

        Ok(created.into())
    }

}