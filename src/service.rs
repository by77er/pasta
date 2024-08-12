use crate::repo::{PasteRepository, RepositoryError};
use async_trait::async_trait;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::sync::RwLock;
use thiserror::Error;
use crate::domain::{Paste, PasteError};

#[derive(Debug, Error)]
pub enum PasteServiceError {
    #[error("content was too long")]
    ContentTooLarge,
    #[error("invalid slug provided")]
    InvalidSlug,
    #[error("internal error occurred")]
    InternalError,
}

pub enum DeleteResult {
    Present,
    NotPresent,
}

#[async_trait]
pub trait PasteService: Send + Sync {
    async fn get_paste(&self, slug: &str) -> Result<Option<String>, PasteServiceError>;
    async fn create_paste(&self, content: String) -> Result<String, PasteServiceError>;
    async fn delete_paste(&self, slug: &str) -> Result<DeleteResult, PasteServiceError>;
}

/// In-memory implementation of paste service
pub struct InMemoryPasteService {
    pastes: RwLock<HashMap<String, String>>,
}

impl InMemoryPasteService {
    pub fn new() -> Self {
        Self {
            pastes: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl PasteService for InMemoryPasteService {
    async fn get_paste(&self, slug: &str) -> Result<Option<String>, PasteServiceError> {
        self.pastes
            .read()
            .map(|map| map.get(slug).cloned())
            .map_err(|_| PasteServiceError::InternalError)
    }

    async fn create_paste(&self, content: String) -> Result<String, PasteServiceError> {
        self.pastes
            .write()
            .map(|mut map| loop {
                let slug = thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(5)
                    .map(char::from)
                    .collect::<String>();
                if map.get(&slug).is_some() {
                    continue;
                }
                map.insert(slug.clone(), content);
                return slug;
            })
            .map_err(|_| PasteServiceError::InternalError)
    }

    async fn delete_paste(&self, slug: &str) -> Result<DeleteResult, PasteServiceError> {
        self.pastes
            .write()
            .map(|mut map| match map.remove(slug) {
                None => DeleteResult::NotPresent,
                Some(_) => DeleteResult::Present,
            })
            .map_err(|_| PasteServiceError::InternalError)
    }
}

///
pub struct RepoBackedPasteService {
    repository: Box<dyn PasteRepository>,
}

impl RepoBackedPasteService {
    pub fn new(repository: Box<dyn PasteRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl PasteService for RepoBackedPasteService {
    async fn get_paste(&self, slug: &str) -> Result<Option<String>, PasteServiceError> {
        let paste = self.repository.get_paste(slug).await?;
        Ok(paste.map(|p| p.get_content().to_string()))
    }

    async fn create_paste(&self, content: String) -> Result<String, PasteServiceError> {
        let paste = Paste::new(content)?;
        let saved_paste = self.repository.save_paste(&paste).await?;
        Ok(saved_paste.get_slug().unwrap().to_string())

    }

    async fn delete_paste(&self, slug: &str) -> Result<DeleteResult, PasteServiceError> {
        let result = self.repository.delete_paste(slug).await?;
        if result > 0 {
            Ok(DeleteResult::Present)
        } else {
            Ok(DeleteResult::NotPresent)
        }
    }
}

impl From<PasteError> for PasteServiceError {
    fn from(value: PasteError) -> Self {
        match value {
            PasteError::ContentTooLarge => PasteServiceError::ContentTooLarge
        }
    }
}

impl From<RepositoryError> for PasteServiceError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::UnknownError => PasteServiceError::InternalError
        }
    }
}
