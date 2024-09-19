use std::path::{Path, PathBuf};
use tokio::{fs::OpenOptions, io::AsyncWriteExt};
use anyhow::{Ok, Result, Error};

pub struct Record {
    pub path: PathBuf,
}


impl Record {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().into()
        }
    }

    pub async fn store(&self, content: (u32, u32, &str)) -> Result<()> {
        let fp = self.path.join(format!("result-{}", content.0));

        let l = format!("{}, {}\n", content.1, content.2);

        let mut f = OpenOptions::new().append(true).create(true).open(&fp).await?;

        f.write(l.as_bytes()).await?;
        
        Ok(())
    }
}


 
