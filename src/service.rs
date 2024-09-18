use std::sync::Arc;
use anyhow::{Ok, Result};

use crate::Model;
use crate::dataset::atk::Atkgen;
use crate::dataset::record::Record;
use crate::utils::get_id;


pub struct AtSrv {
    judge: Arc<Model>,
    target: Arc<Model>,
    atk: Arc<tokio::sync::Mutex<Atkgen>>,
    is_running: Arc<tokio::sync::Mutex<bool>>,
    record: Arc<tokio::sync::Mutex<Record>>,

}


impl AtSrv {
    pub fn new(judge_url: &str, target_url: &str, dataset_path: &str) -> Self {
        Self {
            judge: Arc::new(Model::new(judge_url)),
            target: Arc::new(Model::new(target_url)),
            atk: Arc::new(tokio::sync::Mutex::new(Atkgen::new(dataset_path))),
            is_running: Arc::new(tokio::sync::Mutex::new(false)),
            record: Arc::new(tokio::sync::Mutex::new(Record::new(dataset_path)))
        }
    }


    pub async fn run(&mut self) -> Result<()> {
        *self.is_running.lock().await = true;
        let mut futs = Vec::new();
        for i in 0..10 {
            let ttg = Arc::clone(&self.target);
            let tjd = Arc::clone(&self.judge);
            let isr2 = Arc::clone(&self.is_running);
            let tatk = Arc::clone(&self.atk);
            let ah = tokio::spawn(async move {
                loop {
                    if !*isr2.lock().await {
                        break;
                    }
                    let id = get_id();
                    let prompt = tatk.lock().await.fetch(id).await?;

                    let answer = ttg.generate(&prompt).await?;
                    let judgement = tjd.generate(&answer).await?;
                    println!("{} receive, {}", i, judgement);
                    
                }
                Ok(())
            });
            futs.push(ah);
        }
        for fut in futs {
            let _ = fut.await?;
        }
        Ok(())
    }
}
