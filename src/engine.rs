use crate::{
    console::*,
    db,
    rpc::AuthToken,
    {RPCIP, RPCPASSWORD, RPCPORT, RPCUSER},
};
use bitcoincore_zmq::{subscribe_single_async, Message, Message::HashBlock};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::error::Error;
use surrealdb::{engine::local::Db, Surreal};

pub async fn run() {
    let auth = AuthToken::new()
        .target(RPCIP, RPCPORT, "")
        .credentials(RPCUSER, RPCPASSWORD);
    let db = db::init().await.unwrap();
    if let Err(e) = catchup(&db, &auth).await {
        error!("{}", e);
        std::process::exit(1);
    }
    if let Err(e) = listen(&db, &auth).await {
        error!("{}", e);
        std::process::exit(1);
    }
}

async fn scan(
    blockhash: &String,
    proposal_ids: &mut Vec<u64>,
    db: &Surreal<Db>,
    auth: &AuthToken,
) -> Result<(), Box<dyn Error>> {
    let blockdata: BlockData = getblock(blockhash, &auth).await?;
    if let Ok(Some(proposal)) = getnewproposal(&blockdata, &proposal_ids, &auth).await {
        db::regproposal(&db, &proposal).await?;
        *proposal_ids = db::getproposalids(&db).await?;
    }
    db::regblock(&db, &blockdata).await?;
    Ok(())
}

async fn catchup(db: &Surreal<Db>, auth: &AuthToken) -> Result<(), Box<dyn Error>> {
    let nextheight = match db::toprec(&db).await? {
        Some(thing) => thing + 1,
        None => 0,
    };
    let mut proposal_ids = db::getproposalids(&db).await?;
    for height in nextheight.. {
        let blockhash_result = getblockhash(height, auth).await;
        match blockhash_result {
            Ok(blockhash) => {
                scan(&blockhash, &mut proposal_ids, &db, &auth).await?;
            }
            Err(_) => {
                trace!("Caught up the blocks. Switching to listen mode ...");
                break;
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcessedBlocks {
    pub blocks: Vec<String>,
}

impl ProcessedBlocks {
    fn contains(&self, blockhash: &str) -> bool {
        self.blocks.iter().any(|e| blockhash == e)
    }
    fn inject(&mut self, blockhash: String) {
        self.blocks.push(blockhash);
        // ZMQ by default queues maximum 1000 transactions.
        if self.blocks.len() == 1001 {
            self.blocks.remove(0);
        }
    }
}

async fn listen(db: &Surreal<Db>, auth: &AuthToken) -> Result<(), Box<dyn Error>> {
    let mut proposal_ids = db::getproposalids(&db).await?;
    let mut processed_blocks = ProcessedBlocks::default();
    if let Some(blocks) = db::gettrackedzmq(&db).await? {
        processed_blocks = blocks;
    }

    let mut stream = subscribe_single_async("tcp://127.0.0.1:28332")?;
    while let Some(msg) = stream.next().await {
        let blockhash = gethash(msg);
        if !processed_blocks.contains(&blockhash) {
            scan(&blockhash, &mut proposal_ids, &db, auth).await?;
            processed_blocks.inject(blockhash);
            db::regtrackedzmq(&db, &processed_blocks).await?;
        }
    }
    Ok(())
}

fn gethash<E: Error + Sized>(msg: Result<Message, E>) -> String {
    match msg {
        Ok(msg) => match msg {
            HashBlock(hash, _) => {
                return hash.to_string();
            }
            _ => {
                error!("Got unexpected value from ZMQ.");
                std::process::exit(1);
            }
        },
        Err(e) => {
            error!("{}", e);
            std::process::exit(1);
        }
    }
}