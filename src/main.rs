mod model;
mod service;
mod dataset;
mod config;
mod utils;
mod fy;

use model::model::Model;
use anyhow::Result;
use config::get_config;

#[tokio::main]
async fn main() -> Result<()> {
    let mut srv = service::AtSrv::new(&get_config().judge_url, &get_config().target_url, &get_config().dataset_path, get_config().fy_port);
    srv.run().await
}
