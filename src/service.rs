use std::sync::{atomic::AtomicBool, Arc};
use tokio::sync::{mpsc::{channel, Receiver, Sender}, Mutex};
use anyhow::{Ok, Result, Error};

use crate::Model;
use crate::dataset::atk::Atkgen;
use crate::dataset::record::Record;
use crate::utils::get_id;
use crate::fy::{Fy, Task};

pub struct AtSrv {
    judge: Arc<Model>,
    atk: Arc<Mutex<Atkgen>>,
    is_running: Arc<AtomicBool>,
    record: Arc<Record>,
    fy: Fy,
    fy_r: Arc<Mutex<Receiver<Task>>>,
    target_response_s: Sender<(u32, u32, String)>,
    target_response_r: Arc<Mutex<Receiver<(u32, u32, String)>>>
}


impl AtSrv {
    pub fn new(judge_url: &str, dataset_path: &str, fy_port: u32) -> Self {
        let (fy_s, fy_r) = channel(100);
        let (tr_s, tr_r) = channel(100);
        Self {
            judge: Arc::new(Model::new(judge_url)),
            atk: Arc::new(Mutex::new(Atkgen::new(dataset_path))),
            is_running: Arc::new(AtomicBool::new(false)),
            record: Arc::new(Record::new(dataset_path)),
            fy: Fy::new(fy_port, fy_s),
            fy_r: Arc::new(Mutex::new(fy_r)),
            target_response_s: tr_s,
            target_response_r: Arc::new(Mutex::new(tr_r)),
        }
    }


    pub async fn run(&self) -> Result<()> {


        self.ask_srv().await?;

        self.judge_srv().await?;


        
        self.fy.run().await?;
        Ok(())
    }

    async fn judge_srv(&self) -> Result<()> {
        let tis = Arc::clone(&self.is_running);
        let tr_r = Arc::clone(&self.target_response_r);
        let tj = Arc::clone(&self.judge);
        let trcd = Arc::clone(&self.record);
        let _  = tokio::spawn(async move {
            loop {
                if tis.load(std::sync::atomic::Ordering::SeqCst) {
                    break;
                }
                let response = tr_r.lock().await.recv().await;
                match response {
                    Some(response) => {
                        println!("receive response {}", response.2);
                        let judge = tj.generate(&response.2).await?;
                        println!("receive judge {}", judge);
                        trcd.store((response.0, response.1, &judge)).await?;
                    }
                    None => {
                        
                    }
                }
            }
            Ok(())
        });
        Ok(())
    }
        
    async fn ask_srv(&self) -> Result<()> {
        let tis = Arc::clone(&self.is_running);
        let tts = Arc::clone(&self.fy_r);
        let atk = Arc::clone(&self.atk);
        let tr_s = self.target_response_s.clone();
        let trcd = Arc::clone(&self.record);
        let _ = tokio::spawn(async move {
            loop {
                if tis.load(std::sync::atomic::Ordering::SeqCst) {
                    break;
                }
                let tk = tts.lock().await.recv().await;
                let tatk = Arc::clone(&atk);
                let ttr_s = tr_s.clone();
                let ttrcd = Arc::clone(&trcd);
                match tk {
                    Some(tk) => {
                        let _ = tokio::spawn(async move {
                            let u = Arc::new(Model::new(&tk.target_url));
                            loop {
                                let prompt = tatk.lock().await.fetch(tk.id).await?;
                                println!("fetch prompt {}", prompt);
                                if prompt.len() == 0 {
                                    println!("task finish");
                                    break;
                                }
                                let item_id = get_id();
                                let response = u.generate(&prompt).await?;
                                ttrcd.store((tk.id, item_id, &prompt)).await?;
                                ttrcd.store((tk.id, item_id, &response)).await?;
                                ttr_s.send((tk.id, item_id, response)).await?;
                            }
                            Ok(())
                        });
                    }
                    None => {
                        panic!("abc");
                    }
                }
            }
            Ok(())
        });
        Ok(())
    }
}
