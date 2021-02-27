use std::sync::Arc;

use tokio::sync::RwLock;

pub mod youtrack_service;
pub mod webhook_service;
pub mod grok_service;
pub mod pattern_builder_service;
pub mod definitions;
pub mod operation_service;

pub type Service<T> = Arc<RwLock<T>>;

pub fn new_service<T>(value: T) -> Service<T> where T: Sized {
    let lock = RwLock::new(value);
    Arc::new(lock)
}