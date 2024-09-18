use std::sync::{atomic::AtomicBool, Arc};
use tokio::sync::{mpsc::{channel, Receiver}, Mutex};
use anyhow::{Ok, Result, Error};

use crate::Model;
use crate::dataset::atk::Atkgen;
use crate::dataset::record::Record;
use crate::utils::get_id;
use crate::fy::{Fy, Input};

pub struct AtSrv {
    judge: Arc<Model>,
    target: Arc<Model>,
    atk: Arc<Mutex<Atkgen>>,
    is_running: Arc<AtomicBool>,
    record: Arc<Mutex<Record>>,
    fy: Fy,
    fy_r: Arc<Mutex<Receiver<Input>>>
}


impl AtSrv {
    pub fn new(judge_url: &str, target_url: &str, dataset_path: &str, fy_port: u32) -> Self {
        let (tx, rx) = channel(100);
        Self {
            judge: Arc::new(Model::new(judge_url)),
            target: Arc::new(Model::new(target_url)),
            atk: Arc::new(Mutex::new(Atkgen::new(dataset_path))),
            is_running: Arc::new(AtomicBool::new(false)),
            record: Arc::new(Mutex::new(Record::new(dataset_path))),
            fy: Fy::new(fy_port, tx),
            fy_r: Arc::new(Mutex::new(rx))
        }
    }


    pub async fn run(&mut self) -> Result<()> {
        let tts = Arc::clone(&self.fy_r);
        let tatk = Arc::clone(&self.atk);
        let tis = Arc::clone(&self.is_running);
        let tis1 = Arc::clone(&self.is_running);
        let (tx, mut rx) = channel(100);
        let fh = tokio::spawn(async move {
            loop {
                if tis.load(std::sync::atomic::Ordering::SeqCst) {
                    break;
                }
                let tk = tts.lock().await.recv().await;
                let ttatk = Arc::clone(&tatk);
                let tid = get_id();
                let ttx = tx.clone();
                match tk {
                    Some(tk) => {
                        tokio::spawn(async move {
                            let u = Arc::new(Model::new(&tk.url));
                            loop {
                                let prompt = ttatk.lock().await.fetch(tid).await?;
                                if prompt.len() == 0 {
                                    break;
                                }
                                let response = u.generate(&prompt).await?;
                                ttx.send(response).await?;
                            }
                            Ok(())
                        }).await?;
                    }
                    None => {
                        panic!("abc");
                    }
                }
            }
            Ok(())
        });

        let tj = Arc::clone(&self.judge);

        let jh = tokio::spawn(async move {
            loop {
                if tis1.load(std::sync::atomic::Ordering::SeqCst) {
                    break;
                }
                let response = rx.recv().await;
                match response {
                    Some(response) => {
                        let judge = tj.generate(&response).await?;
                    }
                    None => {
                        
                    }
                }
            }
            Ok(())
        });
        
        self.fy.run().await?;
        fh.await?;
        jh.await?;     
        Ok(())
    }
}
