mod server; 
mod schema;
mod prisma;

use prisma::PrismaClient;
use prisma_client_rust::NewClientError;

extern crate log;
extern crate pretty_env_logger;

#[tokio::main]
pub async fn main() {
    
    let _client: Result<PrismaClient, NewClientError> = PrismaClient::_builder().build().await;

    pretty_env_logger::init();
    
    server::start(([127,0,0,1], 3000)).await;
}