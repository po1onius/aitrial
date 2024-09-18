use tokio::{io::{AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader, Lines}, sync::mpsc};
use std::collections::HashMap;
use anyhow::{Result, Ok, Error};
use tokio::fs::{File, OpenOptions};
use std::path::{PathBuf, Path};

pub struct Atkgen {
    fst: HashMap<u32, Lines<BufReader<File>>>,
    path: PathBuf
}

impl Atkgen {
    pub fn new(p: impl AsRef<Path>) -> Self {
        Self {
            fst: HashMap::new(),
            path: p.as_ref().to_owned()
        }
    }
    
    pub async fn fetch(&mut self, task_id: u32) -> Result<String> {
        match self.fst.get_mut(&task_id) {
            Some(f) => {
                match f.next_line().await? {
                    Some(l) => {
                        Ok(l)
                    }
                    None => {
                        Ok("fallback".to_string())
                    }
                }
            }
            None => {
                let fp = self.path.join(task_id.to_string().as_str());
                let mut f = OpenOptions::new().read(true).write(true).create(true).truncate(true).open(&fp).await?;
                for _ in 0..30 {
                    let _ = f.write("abc\n".as_bytes()).await?;
                }
                f.seek(std::io::SeekFrom::Start(0)).await?;
                let br = BufReader::new(f);
                let mut lines = br.lines();
                let re = lines.next_line().await?;
                self.fst.insert(task_id, lines);
                match re {
                    Some(l) => {
                        Ok(l)
                    }
                    None => {
                        Err(Error::msg("lines error"))
                    }
                }
            }
        }
    }
}
