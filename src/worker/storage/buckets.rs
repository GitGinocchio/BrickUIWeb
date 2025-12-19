#[derive(Debug, Clone)]
pub enum Bucket {
    BrickCode,
    BrickMeta,
    BrickMedia,
    UserMedia,
}

impl Bucket {
    pub fn as_str(&self) -> &'static str {
        match self {
            Bucket::BrickCode => "bricks-code",
            Bucket::BrickMeta => "bricks-meta",
            Bucket::BrickMedia => "bricks-media",
            Bucket::UserMedia => "users-media",
        }
    }
}
