use async_graphql::{Context, InputObject, Object, Result};

use crate::{
    graphql::types::Card,
    prisma::{ deck, PrismaClient},
};

#[derive(InputObject)]
pub struct CreateCardInput {
    pub deck_id: i32,
    pub question: String,
    pub answer: String,
    pub difficulty: i32,
}

#[derive(Default)]
pub struct CardMutation;

#[Object]
impl CardMutation {
    pub async fn create_card(&self, ctx: &Context<'_>, input: CreateCardInput) -> Result<Card> {
        let db = ctx.data::<PrismaClient>().unwrap();

        let created = db
            .card()
            .create(
                deck::id::equals(input.deck_id),
                input.question.to_string(),
                input.answer.to_string(),
                input.difficulty,
                vec![]       
            )
            .exec()
            .await?;

        Ok(created.into())
    }

}