use async_graphql::{ComplexObject, Context, Result, SimpleObject};

use crate::prisma::{deck, user, card, PrismaClient};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub id: i32,
    pub display_name: String,
    pub email: String,
    pub password: String,
}

#[ComplexObject]
impl User {
    pub async fn decks(&self, ctx: &Context<'_>) -> Result<Vec<Deck>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        Ok(db
            .deck()
            .find_many(vec![deck::user_id::equals(self.id.clone())])
            .exec()
            .await?
            .into_iter()
            .map(|p| p.into())
            .collect())
    }

}
impl Into<User> for user::Data {
    fn into(self) -> User {
        User {
            id: self.id,
            display_name: self.display_name,
            email: self.email,
            password: self.password,
        }
    }
}

impl Into<User> for &user::Data {
    fn into(self) -> User {
        User {
            id: self.id.clone(),
            display_name: self.display_name.clone(), 
            email: self.email.clone(),
            password: self.password.clone(),
        }
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Deck {
    pub id:  i32,
    pub user_id:  i32,
    pub title: String,
}

#[ComplexObject]
impl Deck {
    pub async fn user(&self, ctx: &Context<'_>) -> Result<Option<Box<User>>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        Ok(db
            .user()
            .find_unique(user::id::equals(self.user_id.clone()))
            .exec()
            .await?
            .map(|u| Box::new(u.into())))
    }
    pub async fn cards(&self, ctx: &Context<'_>) -> Result<Vec<Card>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        Ok(db
            .card()
            .find_many(vec![card::deck_id::equals(self.id.clone())])
            .exec()
            .await?
            .into_iter()
            .map(|p| p.into())
            .collect())
    }
}

impl Into<Deck> for deck::Data {
    fn into(self) -> Deck {
        Deck {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
        }
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Card {
    pub id:  i32,
    pub deck_id:  i32,
    pub question: String,
    pub answer: String,
    pub difficulty: i32
}

#[ComplexObject]
impl Card {

    pub async fn deck(&self, ctx: &Context<'_>) -> Result<Option<Box<Deck>>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        Ok(db
            .deck()
            .find_unique(deck::id::equals(self.deck_id.clone()))
            .exec()
            .await?
            .map(|u| Box::new(u.into())))
    }
}

impl Into<Card> for card::Data {
    fn into(self) -> Card {
        Card {
            id: self.id,
            deck_id: self.deck_id,
            question: self.question,
            answer: self.answer,
            difficulty: self.difficulty,
        }
    }
}
