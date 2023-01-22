use async_graphql::{Context, Object, Result};

use crate::{graphql::types::Card, prisma::PrismaClient};

#[derive(Default)]
pub struct CardQuery;

#[Object]
impl CardQuery {
    async fn get_cards(&self, ctx: &Context<'_>) -> Result<Vec<Card>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        Ok(db
            .card()
            .find_many(vec![])
            .exec()
            .await?
            .into_iter()
            .map(|p| p.into())
            .collect())
    }
}