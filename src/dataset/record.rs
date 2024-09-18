use std::path::{Path, PathBuf};

use anyhow::Result;

pub struct Record {
    pub path: PathBuf,        
}


impl Record {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().into()
        }
    }

    pub async fn store(&mut self, content: (u32, &str)) -> Result<()> {
        
    }
}


 
