use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::sql::Thing;
use surrealdb::Surreal;

use crate::console::{BlockData, Proposal};
use crate::{DATABASE, STAGE};

pub async fn init() -> surrealdb::Result<Surreal<Db>> {
    info!("Creating {} ...", DATABASE);
    let db = Surreal::new::<RocksDb>(DATABASE).await?;
    db.use_ns(STAGE).use_db(STAGE).await?;
    Ok(db)
}

pub async fn toprec(db: &Surreal<Db>) -> Option<u64> {
    info!("Querying the top record height ...");
    let mut response = db
        .query("math::max(SELECT VALUE height FROM blocks)")
        .await
        .unwrap();
    let toprec: Option<u64> = response.take(0).unwrap();
    match toprec {
        Some(height) => {
            debug!("Database sanity check ...");
            trace!("Folding on the Rust side ...");
            let fold: u64 = (0..=height).fold(0, |acc, x| acc + x);
            trace!("Folding on the database side ...");
            let dbfold: Option<u64> = db
                .query("math::sum(SELECT VALUE height FROM blocks)")
                .await
                .unwrap()
                .take(0)
                .unwrap();
            /*if fold != dbfold.unwrap() {
                error!(
                    "Database is insane! Rust fold: {}, SurrealDB fold: {}",
                    fold,
                    dbfold.unwrap()
                );
                std::process::exit(1);
            }*/
        }
        _ => {}
    }

    return toprec;
}

pub async fn getproposalids(db: &Surreal<Db>) -> Vec<u64> {
    info!("Querying proposals ...");
    let mut response = db
        .query("SELECT VALUE proposal_id FROM proposals")
        .await
        .unwrap();
    let proposal_ids: Vec<u64> = response.take(0).unwrap();
    return proposal_ids;
}

pub async fn regblock(db: &Surreal<Db>, blockdata: &BlockData) {
    info!(
        "Registering block {} into DB '{}' ...",
        blockdata.height, STAGE
    );
    let x: Vec<BlockData> = db.create("blocks").content(blockdata).await.unwrap();
}

pub async fn regproposal(db: &Surreal<Db>, proposal: &Proposal) {
    info!(
        "Registering proposal ID {} into DB '{}' ...",
        proposal.proposal_id, STAGE
    );
    let x: Vec<Proposal> = db.create("proposals").content(proposal).await.unwrap();
}
