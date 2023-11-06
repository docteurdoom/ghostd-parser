#![allow(unused)]
#[macro_use]
extern crate log;
pub const CRATE_NAME: &str = module_path!();
mod console;
mod db;
mod logger;
mod pools;
mod rpc;
use crate::console::*;
use rpc::AuthToken;

pub const RPCIP: &str = "127.0.0.1";
pub const RPCPORT: u32 = 51725;
pub const RPCUSER: &str = "user";
pub const RPCPASSWORD: &str = "password";
pub const DATABASE: &str = "/home/user/.local/share/ghost-parser-prod.db";
pub const STAGE: &str = "prod";

#[tokio::main]
async fn main() {
    logger::init();
    let authtoken = AuthToken::new()
        .target(RPCIP, RPCPORT, "")
        .credentials(RPCUSER, RPCPASSWORD);
    let db = db::init().await.unwrap();
    
    //let blockdata: BlockData = getblock(799907, &authtoken).await;
    let nextheight = match db::toprec(&db).await {
        Some(thing) => thing + 1,
        None => 0,
    };
    let mut proposal_ids = db::getproposalids(&db).await;
    let currentheight = getblockcount(&authtoken).await;
    for height in nextheight..=currentheight {
        let blockdata: BlockData = getblock(height, &authtoken).await;
        if let Some(proposal) = getnewproposal(&blockdata, &proposal_ids, &authtoken).await {
            db::regproposal(&db, &proposal).await;
            proposal_ids = db::getproposalids(&db).await;
        }
        db::regblock(&db, &blockdata).await;
    }
}
