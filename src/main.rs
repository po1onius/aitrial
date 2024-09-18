mod model;
mod service;
mod dataset;
mod config;
mod utils;

use model::model::Model;
use anyhow::Result;
use config::get_config;

#[tokio::main]
async fn main() -> Result<()> {
    let mut srv = service::AtSrv::new(&get_config().judge_url, &get_config().target_url, &get_config().dataset_path);
    srv.run().await
}
