use axum::{
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap, StatusCode},
};
use serde::Deserialize;
use std::{collections::HashSet, env, sync::Arc};

#[derive(Clone)]
pub struct AdminAuthState {
    pub supabase_url: String,
    pub anon_key: String,
    pub allowed_emails: HashSet<String>,
}

impl AdminAuthState {
    pub fn from_env() -> Result<Self, String> {
        let supabase_url = env::var("PUBLIC_SUPABASE_URL")
            .map_err(|_| "Missing PUBLIC_SUPABASE_URL".to_string())?;
        let anon_key = env::var("PUBLIC_SUPABASE_PUBLISHABLE_DEFAULT_KEY")
            .map_err(|_| "Missing PUBLIC_SUPABASE_PUBLISHABLE_DEFAULT_KEY".to_string())?;

        let allowed = env::var("ADMIN_ALLOWED_EMAILS")
            .unwrap_or_else(|_| "".to_string())
            .split(',')
            .map(|email| email.trim().to_lowercase())
            .filter(|email| !email.is_empty())
            .collect::<HashSet<_>>();

        Ok(Self {
            supabase_url,
            anon_key,
            allowed_emails: allowed,
        })
    }
}

#[derive(Deserialize)]
struct SupabaseUser {
    email: Option<String>,
}

pub struct AdminUser {
    pub email: String,
}

impl FromRequestParts<Arc<AdminAuthState>> for AdminUser {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        _parts: &mut Parts,
        _state: &Arc<AdminAuthState>,
    ) -> Result<Self, Self::Rejection> {
        // AUTH DISABLED - allowing all access
        Ok(AdminUser { email: "admin@avvai.edu".to_string() })
    }
}

fn extract_bearer(headers: &HeaderMap) -> Option<String> {
    let header = headers.get("Authorization")?.to_str().ok()?;
    let mut parts = header.split_whitespace();
    let scheme = parts.next()?;
    let token = parts.next()?;
    if scheme.eq_ignore_ascii_case("Bearer") {
        Some(token.to_string())
    } else {
        None
    }
}
