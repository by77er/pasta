use crate::service::InMemoryPasteService;

mod controller;
mod repo;
mod service;

#[tokio::main]
pub async fn main() {
    let paste_service = Box::new(InMemoryPasteService::new());
    controller::listen(paste_service).await;
}
