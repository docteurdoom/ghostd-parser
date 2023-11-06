use crate::{
    console::Vout::Data,
    pools::{Pool, POOLS},
    rpc::{call, AuthToken},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockData {
    pub bits: String,
    pub blocksig: Option<String>,
    pub chainwork: String,
    pub difficulty: f64,
    pub hash: String,
    pub hashproofofstake: Option<String>,
    pub height: u64,
    pub mediantime: u64,
    pub merkleroot: String,
    #[serde(rename(deserialize = "nTx", serialize = "nTx"))]
    pub n_tx: u64,
    pub nonce: u64,
    pub previousblockhash: Option<String>,
    pub prevstakemodifier: Option<String>,
    pub size: u64,
    pub stakekernelblockhash: Option<String>,
    pub stakekernelscript: Option<String>,
    pub stakekernelvalue: Option<f64>,
    pub strippedsize: u64,
    pub time: u64,
    pub tx: Vec<Transaction>,
    pub version: u64,
    #[serde(rename(deserialize = "versionHex", serialize = "versionHex"))]
    pub version_hex: String,
    pub weight: u64,
    pub witnessmerkleroot: String,
    pub pool_info: Option<Pool>,
    pub voting_info: Option<Vote>,
}

impl BlockData {
    async fn validateaddress(&mut self, auth: &AuthToken) -> Result<(), Box<dyn Error>> {
        let hasstakeaddress: Option<Vec<String>> = match self.tx[0].vout[1].clone() {
            Vout::Standard {
                n: _,
                vout_type: _,
                value: _,
                valuesat: _,
                scriptpubkey,
            } => scriptpubkey.stakeaddresses,
            _ => {
                error!("Unexpected type of vout when validating address.");
                std::process::exit(1);
            }
        };
        match hasstakeaddress {
            Some(stakeaddress) => {
                let arg = format!("validateaddress {} true", &stakeaddress[0]);
                let value = call(&arg, auth).await?;
                let mut pool: Option<Pool> = None;
                for known_pool in POOLS {
                    if &value["stakeonly_address"] == known_pool.pubkey {
                        pool = Some(known_pool.makepub());
                        break;
                    }
                }
                self.pool_info = pool;
                Ok(())
            }
            None => {
                self.pool_info = None;
                Ok(())
            }
        }
    }
    fn read_vote(&mut self) {
        let vout = self.tx[0].vout[0].clone();
        match vout {
            Data {
                n: _,
                data_hex: _,
                smsgdifficulty: _,
                smsgfeerate: _,
                treasury_fund_cfwd: _,
                vout_type: _,
                vote,
            } => match vote {
                Some(content) => {
                    let parsed: Vec<u64> = content
                        .split(", ")
                        .map(|x| x.parse::<u64>().unwrap())
                        .collect();
                    if parsed.len() != 2 {
                        error!("Sanity checks for parsed vote stats failed.");
                        std::process::exit(1);
                    }
                    self.voting_info = Some(Vote {
                        proposal_id: parsed[0],
                        voted_for_option: parsed[1],
                    });
                }
                None => {
                    self.voting_info = None;
                }
            },
            _ => {
                self.voting_info = None;
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub txid: String,
    pub hash: String,
    pub version: u64,
    pub size: u64,
    pub vsize: u64,
    pub weight: u64,
    pub locktime: u64,
    pub hex: String,
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Vout {
    Data {
        n: u64,
        data_hex: String,
        smsgdifficulty: Option<String>,
        smsgfeerate: Option<f64>,
        treasury_fund_cfwd: Option<f64>,
        #[serde(rename(deserialize = "type", serialize = "type"))]
        vout_type: String,
        vote: Option<String>,
    },
    Standard {
        n: u64,
        #[serde(rename(deserialize = "type", serialize = "type"))]
        vout_type: String,
        value: f64,
        #[serde(rename(deserialize = "valueSat", serialize = "valueSat"))]
        valuesat: u64,
        #[serde(rename(deserialize = "scriptPubKey", serialize = "scriptPubKey"))]
        scriptpubkey: ScriptPubKey,
    },
    Blind {
        n: u64,
        #[serde(rename(deserialize = "type", serialize = "type"))]
        vout_type: String,
        pubkey: Option<String>,
        #[serde(rename(deserialize = "valueCommitment", serialize = "valueCommitment"))]
        value_commitment: String,
        data_hex: String,
        rangeproof: String,
    },
    Anon {
        n: u64,
        #[serde(rename(deserialize = "type", serialize = "type"))]
        vout_type: String,
        pubkey: Option<String>,
        #[serde(rename(deserialize = "valueCommitment", serialize = "valueCommitment"))]
        value_commitment: String,
        data_hex: String,
        rangeproof: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptPubKey {
    pub addresses: Option<Vec<String>>,
    pub stakeaddresses: Option<Vec<String>>,
    pub asm: String,
    pub hex: String,
    #[serde(rename(deserialize = "reqSigs", serialize = "reqSigs"))]
    pub req_sigs: Option<u64>,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub staking_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Vin {
    Standard {
        txid: String,
        vout: u64,
        #[serde(rename(deserialize = "scriptSig", serialize = "scriptSig"))]
        script_sig: ScriptSig,
    },
    Anon {
        #[serde(rename(deserialize = "type", serialize = "type"))]
        input_type: String,
        num_inputs: u64,
        ring_size: u64,
        txinwitness: Vec<String>,
        sequence: u64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSig {
    pub asm: String,
    pub hex: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConclusion {
    pub isvalid: bool,
    pub stakeonly_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub proposal_id: u64,
    pub voted_for_option: u64,
}

impl Vote {
    pub async fn gen_proposal(&self, auth: &AuthToken) -> Result<Proposal, Box<dyn Error>> {
        Ok(Proposal {
            proposal_id: self.proposal_id,
            stats: self.count_stats(auth).await?,
        })
    }
    async fn count_stats(
        &self,
        auth: &AuthToken,
    ) -> Result<HashMap<String, (u64, f64)>, Box<dyn Error>> {
        Ok(tallyvotes(*&self.proposal_id, auth).await?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub proposal_id: u64,
    pub stats: HashMap<String, (u64, f64)>,
}

async fn tallyvotes(
    proposal_id: u64,
    auth: &AuthToken,
) -> Result<HashMap<String, (u64, f64)>, Box<dyn Error>> {
    let arg = format!("tallyvotes {} 710800 {}", proposal_id, i32::MAX);
    let context = call(&arg, auth).await?;
    let rawmap: HashMap<String, Value> = serde_json::from_value(context)?;
    let mut hmap: HashMap<String, (u64, f64)> = rawmap
        .iter()
        .map(|val| {
            (
                val.0.to_string().replace('"', ""),
                parse_tallyvotes_ratios(val.1.to_string().replace('"', "")),
            )
        })
        .collect();
    hmap.remove_entry("proposal").unwrap();
    hmap.remove_entry("blocks_counted").unwrap();
    hmap.remove_entry("height_start").unwrap();
    hmap.remove_entry("height_end").unwrap();
    Ok(hmap)
}

fn parse_tallyvotes_ratios(raw: String) -> (u64, f64) {
    let vote_stats_iterator = raw.split(", ");
    let mut index = 0;
    let mut vote_args_tuple: (u64, f64) = (0, 0.0);
    for vote_stat in vote_stats_iterator {
        if index == 0 {
            vote_args_tuple.0 = vote_stat.replace("%", "").trim().parse::<u64>().unwrap();
        } else if index == 1 {
            vote_args_tuple.1 = vote_stat.replace("%", "").trim().parse::<f64>().unwrap();
        }
        index += 1;
    }
    return vote_args_tuple;
}

pub async fn getblockchaininfo(auth: &AuthToken) -> Result<Value, Box<dyn Error>> {
    Ok(call("getblockchaininfo", auth).await?)
}

pub async fn getblockcount(auth: &AuthToken) -> Result<u64, Box<dyn Error>> {
    let raw = call("getblockcount", auth).await?;
    let height: u64 = serde_json::from_value(raw)?;
    Ok(height)
}

pub async fn getblockhash(height: u64, auth: &AuthToken) -> Result<String, Box<dyn Error>> {
    let arg = format!("getblockhash {}", height);
    let raw = call(&arg, auth).await?;
    let hash: String = serde_json::from_value(raw)?;
    Ok(hash)
}

pub async fn getblock(
    blockhash: impl Into<String>,
    auth: &AuthToken,
) -> Result<BlockData, Box<dyn Error>> {
    let arg = format!("getblock {} 2 true", blockhash.into());
    let value = call(&arg, auth).await?;
    let mut blockdata: BlockData = serde_json::from_value(value)?;
    blockdata.validateaddress(auth).await;
    blockdata.read_vote();
    Ok(blockdata)
}

pub async fn getnewproposal(
    blockdata: &BlockData,
    proposal_ids: &Vec<u64>,
    auth: &AuthToken,
) -> Result<Option<Proposal>, Box<dyn Error>> {
    if blockdata.height > 710800 {
        match blockdata.voting_info.clone() {
            Some(vote) => {
                let existsyet = proposal_ids.iter().any(|&x| x == vote.proposal_id);
                if !existsyet {
                    let proposal = vote.gen_proposal(auth).await?;
                    Ok(Some(proposal))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}
