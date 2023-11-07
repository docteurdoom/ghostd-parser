#![allow(unused)]
#[macro_use]
extern crate log;
pub const CRATE_NAME: &str = module_path!();
pub use std::error::Error;
mod console;
mod db;
mod engine;
mod logger;
mod pools;
mod rpc;

pub const RPCIP: &str = "127.0.0.1";
pub const RPCPORT: u32 = 51725;
pub const RPCUSER: &str = "user";
pub const RPCPASSWORD: &str = "password";
pub const DATABASE: &str = "/home/user/.local/share/ghost-parser/ghost-parser-prod.db";
pub const STAGE: &str = "prod";

#[tokio::main]
async fn main() {
    logger::init();
    engine::run().await;
}
