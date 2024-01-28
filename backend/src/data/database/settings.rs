use color_eyre::Result;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::service::stash_config::StashConfig;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    pub stash: StashConfig,
}

#[derive(Clone)]
pub struct SettingsDatabase {
    pool: SqlitePool,
}

impl SettingsDatabase {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn fetch_settings(&self) -> Result<Option<Settings>> {
        let settings = sqlx::query!("SELECT * FROM settings WHERE id = 1")
            .fetch_optional(&self.pool)
            .await?
            .map(|row| {
                serde_json::from_str(&row.settings_json)
                    .expect("invalid JSON found in settings database table")
            });

        Ok(settings)
    }

    pub async fn set_settings(&self, settings: Settings) -> Result<()> {
        let settings_json = serde_json::to_string(&settings)?;
        sqlx::query!(
            "INSERT INTO settings (id, settings_json) VALUES (1, ?) ON CONFLICT(id) DO UPDATE SET settings_json = ?",
            settings_json,
            settings_json
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
