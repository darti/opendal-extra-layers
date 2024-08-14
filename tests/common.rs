use ctor::ctor;

use opendal::{services::Fs, Operator};

#[ctor]
fn init() {
    console_subscriber::init();
}

pub trait ListDir {
    async fn entries(&self, path: &str) -> anyhow::Result<Vec<String>>;
}

impl ListDir for Operator {
    async fn entries(&self, path: &str) -> anyhow::Result<Vec<String>> {
        let ds = self.list(path).await?;
        let mut entries: Vec<String> = ds.iter().map(|e| e.name().to_owned()).collect();

        entries.sort();

        Ok(entries)
    }
}
