use crate::service::PasteError::InternalError;
use async_trait::async_trait;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::sync::RwLock;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasteError {
    #[error("content was too long")]
    ContentTooLong,
    #[error("internal error occurred")]
    InternalError,
}

pub enum DeleteResult {
    Present,
    NotPresent,
}

type Slug = String;

#[async_trait]
pub trait PasteService: Send + Sync {
    async fn get_paste(&self, slug: Slug) -> Result<Option<String>, PasteError>;
    async fn create_paste(&self, content: String) -> Result<Slug, PasteError>;
    async fn delete_paste(&self, slug: Slug) -> Result<DeleteResult, PasteError>;
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
    async fn get_paste(&self, slug: Slug) -> Result<Option<String>, PasteError> {
        self.pastes
            .read()
            .map(|map| map.get(&slug).cloned())
            .map_err(|_| InternalError)
    }

    async fn create_paste(&self, content: String) -> Result<Slug, PasteError> {
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
            .map_err(|_| InternalError)
    }

    async fn delete_paste(&self, slug: Slug) -> Result<DeleteResult, PasteError> {
        self.pastes
            .write()
            .map(|mut map| match map.remove(&slug) {
                None => DeleteResult::NotPresent,
                Some(_) => DeleteResult::Present,
            })
            .map_err(|_| InternalError)
    }
}
