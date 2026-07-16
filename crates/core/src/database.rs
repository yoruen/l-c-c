//! Database layer using SQLite

use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::path::Path;

pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(db_path: &Path) -> anyhow::Result<Self> {
        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        let database_url = format!("sqlite://{}", db_path.display());
        
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;
            
        // Run migrations
        Self::run_migrations(&pool).await?;
        
        Ok(Self { pool })
    }
    
    async fn run_migrations(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            
            CREATE TABLE IF NOT EXISTS plugin_settings (
                plugin_id TEXT NOT NULL,
                key TEXT NOT NULL,
                value TEXT NOT NULL,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                PRIMARY KEY (plugin_id, key)
            );
            
            CREATE TABLE IF NOT EXISTS action_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                plugin_id TEXT NOT NULL,
                action_type TEXT NOT NULL,
                details TEXT,
                status TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            
            CREATE TABLE IF NOT EXISTS search_index (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                plugin_id TEXT NOT NULL,
                entity_type TEXT NOT NULL,
                entity_id TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                keywords TEXT,
                UNIQUE(plugin_id, entity_type, entity_id)
            );
            
            CREATE INDEX IF NOT EXISTS idx_search_title ON search_index(title);
            CREATE INDEX IF NOT EXISTS idx_search_keywords ON search_index(keywords);
            CREATE INDEX IF NOT EXISTS idx_action_history ON action_history(created_at);
            "#
        )
        .execute(pool)
        .await?;
        
        Ok(())
    }
    
    pub fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }
}
