use anyhow::Result;



pub trait Badb {
    fn store() -> Result<()>;
    fn fetch() -> Result<String>;
}


