use crate::service::InMemoryPasteService;

mod presentation;
mod service;
mod repo;
mod domain;

#[tokio::main]
pub async fn main() {
    let paste_service = Box::new(InMemoryPasteService::new());
    presentation::listen(paste_service).await;
}
