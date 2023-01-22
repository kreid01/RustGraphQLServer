pub mod user;
pub mod deck;
pub mod card;

pub use user::UserQuery;
pub use deck::DeckQuery;
pub use card::CardQuery;

// Add your other ones here to create a unified Query object
// e.x. Query(PostQuery, OtherQuery, OtherOtherQuery)
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(UserQuery, DeckQuery, CardQuery);