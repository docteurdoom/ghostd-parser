use crate::{
    console::{
        BlockData,
        Proposal
    },
    engine::ProcessedBlocks,
};
use clap::ArgMatches;
use std::error::Error;
use surrealdb::{
    engine::remote::ws::{
        Ws,
        Client
    },
    Surreal
};

pub async fn init(args: &ArgMatches) -> surrealdb::Result<Surreal<Client>> {
    let stage = args.get_one::<String>("stage").unwrap();
    let is_ip: Option<String> = args.get_one::<String>("SurrealDB IP").cloned();
    match is_ip {
        Some(ip) => {
            info!("Connecting {} ...", &ip);
            let db = Surreal::new::<Ws>(ip).await?;
            db.use_ns(stage).use_db(stage).await?;
            return Ok(db);
        }
        None => {
            // Can't be None, because CLAP won't let the program run without this value.
            unreachable!();
        }
    }
}

// Sum heights from bottom to top both
// mathematically and via SQL to ensure data consistency
pub async fn toprec(db: &Surreal<Client>) -> Result<Option<u64>, Box<dyn Error>> {
    debug!("Database sanity check ...");
    trace!("Running a set of queries ...");
    let mut response = db
        .query("math::max(SELECT VALUE height FROM blocks)")
        .query("math::min(SELECT VALUE height FROM blocks)")
        .query("math::sum(SELECT VALUE height FROM blocks)")
        .await?;
    match response.take(0)? {
        Some(top_height) => {
            let min_height = response.take::<Option<u64>>(1)?.unwrap();
            trace!("Lowest height: {}, Top height: {}", min_height, top_height);
            if min_height != 0 {
                warn!(
                    "Lowest height record is {}. Should be 0, unless intentional.",
                    min_height
                );
            }
            let fold: u64 = (min_height..=top_height).fold(0, |acc, x| acc + x);
            let dbfold: u64 = response.take::<Option<u64>>(2)?.unwrap();
            if fold != dbfold {
                error!(
                    "Database is insane! Rust fold: {}, SurrealDB fold: {}",
                    fold, dbfold
                );
                std::process::exit(1);
            }
            Ok(Some(top_height))
        }
        None => {
            trace!("No heights recorded yet.");
            Ok(None)
        }
    }
}

pub async fn getproposalids(db: &Surreal<Client>) -> Result<Vec<u64>, Box<dyn Error>> {
    trace!("Querying proposals ...");
    let mut response = db.query("SELECT VALUE proposal_id FROM proposals").await?;
    let proposal_ids: Vec<u64> = response.take(0)?;
    Ok(proposal_ids)
}

pub async fn gettrackedzmq(
    db: &Surreal<Client>,
) -> Result<Option<ProcessedBlocks>, Box<dyn Error>> {
    trace!("Querying last 1000 ZMQ processed blocks ...");
    let mut response = db.query("SELECT * FROM zmq").await?;
    let zmqueue: Option<ProcessedBlocks> = response.take(0)?;
    Ok(zmqueue)
}

pub async fn regtrackedzmq(
    db: &Surreal<Client>,
    queue: &ProcessedBlocks,
) -> Result<(), Box<dyn Error>> {
    trace!("Recording ZMQ queue for later use ...");
    let _ = db.query("DELETE zmq").await?;
    let _: Vec<ProcessedBlocks> = db.create("zmq").content(queue).await?;
    Ok(())
}

pub async fn regblock(db: &Surreal<Client>, blockdata: &BlockData) -> Result<(), Box<dyn Error>> {
    info!("Registering block {} into DB ...", blockdata.height);
    let _: Option<BlockData> = db.create(("blocks", blockdata.height)).content(blockdata).await?;
    Ok(())
}

pub async fn regproposal(db: &Surreal<Client>, proposal: &Proposal) -> Result<(), Box<dyn Error>> {
    info!(
        "Registering proposal ID {} into DB ...",
        proposal.proposal_id
    );
    let _: Option<Proposal> = db.create(("proposals", proposal.proposal_id)).content(proposal).await?;
    Ok(())
}
