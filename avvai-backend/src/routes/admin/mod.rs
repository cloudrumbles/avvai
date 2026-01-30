pub mod dictionary_cache;
pub mod lessons;
pub mod media;

use axum::Router;
use std::sync::Arc;

use crate::services::admin_auth::AdminAuthState;

pub fn router() -> Router<Arc<AdminAuthState>> {
    Router::new()
        .nest("/dictionary-cache", dictionary_cache::router())
        .nest("/content", lessons::router())
        .nest("/assets", media::router())
}
