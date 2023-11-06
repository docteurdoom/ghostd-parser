use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;

use crate::console::{BlockData, Proposal};
use crate::engine::ProcessedBlocks;
use crate::{DATABASE, STAGE};

pub async fn init() -> surrealdb::Result<Surreal<Db>> {
    info!("Connecting {} ...", DATABASE);
    let db = Surreal::new::<RocksDb>(DATABASE).await?;
    db.use_ns(STAGE).use_db(STAGE).await?;
    Ok(db)
}

pub async fn toprec(db: &Surreal<Db>) -> Option<u64> {
    debug!("Querying the top height ...");
    let mut response = db
        .query("math::max(SELECT VALUE height FROM blocks)")
        .await
        .unwrap();
    let toprec: Option<u64> = response.take(0).unwrap();
    if let Some(height) = toprec {
        sanitycheck(height, db).await;
    }
    return toprec;
}

// Sum heights from bottom to top both
// mathematically and via SQL to ensure data consistency
async fn sanitycheck(top_height: u64, db: &Surreal<Db>) {
    debug!("Database sanity check ...");
    debug!("Querying the minimum height ...");
    let mut response = db
        .query("math::min(SELECT VALUE height FROM blocks)")
        .await
        .unwrap();
    let min_height_option: Option<u64> = response.take(0).unwrap();
    let min_height = min_height_option.unwrap();
    if min_height != 0 {
        warn!(
            "Lowest height record is {}. Should be 0, unless intentional.",
            min_height
        );
    }
    trace!("Folding on the Rust side ...");
    let fold: u64 = (min_height..=top_height).fold(0, |acc, x| acc + x);
    trace!("Folding on the database side ...");
    let dbfold: Option<u64> = db
        .query("math::sum(SELECT VALUE height FROM blocks)")
        .await
        .unwrap()
        .take(0)
        .unwrap();
    if fold != dbfold.unwrap() {
        error!(
            "Database is insane! Rust fold: {}, SurrealDB fold: {}",
            fold,
            dbfold.unwrap()
        );
        std::process::exit(1);
    }
}

pub async fn getproposalids(db: &Surreal<Db>) -> Vec<u64> {
    debug!("Querying proposals ...");
    let mut response = db
        .query("SELECT VALUE proposal_id FROM proposals")
        .await
        .unwrap();
    let proposal_ids: Vec<u64> = response.take(0).unwrap();
    return proposal_ids;
}

pub async fn gettrackedzmq(db: &Surreal<Db>) -> Option<ProcessedBlocks> {
    debug!("Querying last 1000 ZMQ processed blocks ...");
    let mut response = db.query("SELECT * FROM zmq").await.unwrap();
    let zmqueue: Option<ProcessedBlocks> = response.take(0).unwrap();
    return zmqueue;
}

pub async fn regtrackedzmq(db: &Surreal<Db>, queue: &ProcessedBlocks) {
    trace!("Recording ZMQ queue for later use ...");
    let _ = db.query("DELETE zmq").await.unwrap();
    let _: Vec<ProcessedBlocks> = db.create("zmq").content(queue).await.unwrap();
}

pub async fn regblock(db: &Surreal<Db>, blockdata: &BlockData) {
    info!(
        "Registering block {} into DB '{}' ...",
        blockdata.height, STAGE
    );
    let _: Vec<BlockData> = db.create("blocks").content(blockdata).await.unwrap();
}

pub async fn regproposal(db: &Surreal<Db>, proposal: &Proposal) {
    info!(
        "Registering proposal ID {} into DB '{}' ...",
        proposal.proposal_id, STAGE
    );
    let _: Vec<Proposal> = db.create("proposals").content(proposal).await.unwrap();
}
