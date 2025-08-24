use opendal::services::Fs;
use opendal::Operator;

pub struct OpenDalFs {
    op: Operator,
}

impl OpenDalFs {
    pub fn new(root: &str) -> anyhow::Result<Self> {
        let mut builder = Fs::default();
        builder.root(root);
        let op = Operator::new(builder)?.finish();
        Ok(Self { op })
    }

    pub async fn write(&self, path: &str, content: Vec<u8>) -> anyhow::Result<()> {
        self.op.write(path, content).await?;
        Ok(())
    }
} 