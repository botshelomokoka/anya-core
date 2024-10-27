use async_trait::async_trait;

#[async_trait]
pub trait DataFeedTrait {
    async fn get_data(&mut self) -> Option<Vec<f32>>;
    async fn request_data(&mut self);
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum DataSource {
    Market,
    Blockchain,
    SocialMedia,
    // Add other data sources as needed
}

pub struct DataFeed {
    // Fields and methods for DataFeed...
}

pub enum DataSource {
    Market,
    // Other data sources...
}

impl DataFeed {
    // Implementation of DataFeed methods...
}
