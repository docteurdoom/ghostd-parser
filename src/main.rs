//#![allow(unused)]
#[macro_use]
extern crate log;
pub const CRATE_NAME: &str = module_path!();
mod args;
mod console;
mod db;
mod engine;
mod logger;
mod pools;
mod rpc;

pub const DATABASE: &str = "127.0.0.1:8000";
pub const STAGE: &str = "prod";

#[tokio::main]
async fn main() {
    let args = args::args();
    logger::init();
    engine::run(&args).await;
}
