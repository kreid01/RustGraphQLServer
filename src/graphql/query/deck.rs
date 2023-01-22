use async_graphql::{Context, Object, Result};

use crate::{graphql::types::Deck, prisma::PrismaClient};

#[derive(Default)]
pub struct DeckQuery;

#[Object]
impl DeckQuery {
    async fn get_decks(&self, ctx: &Context<'_>) -> Result<Vec<Deck>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        Ok(db
            .deck()
            .find_many(vec![])
            .exec()
            .await?
            .into_iter()
            .map(|p| p.into())
            .collect())
    }
}