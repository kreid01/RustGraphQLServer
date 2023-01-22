pub mod user;
pub mod deck;
pub mod card;

pub use user::UserMutation;
pub use deck::DeckMutation;
pub use card::CardMutation;

// Add your other ones here to create a unified Mutation object
// e.x. Mutation(PostMutation, OtherMutation, OtherOtherMutation)
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(DeckMutation, UserMutation, CardMutation);