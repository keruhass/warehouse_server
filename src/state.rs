use dashmap::DashMap;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub supplier_cache: DashMap<String, String>,
}
