use crate::domain::Paste;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("unknown error occurred")]
    UnknownError,
}

#[async_trait]
pub trait PasteRepository: Send + Sync {
    async fn get_paste(&self, slug: &str) -> Result<Option<Paste>, RepositoryError>;
    async fn save_paste(&self, paste: &Paste) -> Result<Paste, RepositoryError>;
    async fn delete_paste(&self, slug: &str) -> Result<usize, RepositoryError>;
    async fn generate_slug(&self) -> Result<String, RepositoryError>;
}

pub struct PostgresRepository {}

impl PostgresRepository {}

#[async_trait]
impl PasteRepository for PostgresRepository {
    async fn get_paste(&self, slug: &str) -> Result<Option<Paste>, RepositoryError> {
        todo!()
    }

    async fn save_paste(&self, paste: &Paste) -> Result<Paste, RepositoryError> {
        todo!()
    }

    async fn delete_paste(&self, slug: &str) -> Result<usize, RepositoryError> {
        todo!()
    }

    async fn generate_slug(&self) -> Result<String, RepositoryError> {
        todo!()
    }
}
