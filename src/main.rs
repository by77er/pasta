use crate::service::InMemoryPasteService;

mod domain;
mod presentation;
mod repo;
mod service;

#[tokio::main]
pub async fn main() {
    let paste_service = Box::new(InMemoryPasteService::new());
    presentation::listen(paste_service).await;
}
