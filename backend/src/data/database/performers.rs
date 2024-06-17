use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tracing::info;

use crate::Result;

// taken from stash
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "kebab-case")]
pub enum Gender {
    Male,
    Female,
    TransgenderMale,
    TransgenderFemale,
    Intersex,
    NonBinary,
}

#[derive(Debug)]
pub struct DbPerformer {
    pub id: i64,
    pub name: String,
    pub created_on: i64,
    pub image_url: Option<String>,
    pub stash_id: Option<String>,
    pub gender: Option<Gender>,
}

#[derive(Clone, Debug)]
pub struct CreatePerformer {
    pub name: String,
    pub image_url: Option<String>,
    pub stash_id: Option<String>,
    pub gender: Option<Gender>,
}

#[derive(Debug, Clone)]
pub struct PerformersDatabase {
    pool: SqlitePool,
}

impl PerformersDatabase {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<DbPerformer>> {
        sqlx::query_as!(DbPerformer, r#"SELECT rowid AS id, name, created_on, image_url, stash_id, gender AS "gender: Gender" FROM performers"#)
            .fetch_all(&self.pool)
            .await
            .map_err(From::from)
    }

    pub async fn insert(&self, performer: CreatePerformer) -> Result<()> {
        info!("Inserting performer: {:?}", performer);
        sqlx::query!(
            "INSERT INTO performers (name, image_url, stash_id, gender, created_on) 
             VALUES ($1, $2, $3, $4, strftime('%s', 'now'))",
            performer.name,
            performer.image_url,
            performer.stash_id,
            performer.gender,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[sqlx::test]
    async fn insert_performers(pool: SqlitePool) -> Result<()> {
        let db = PerformersDatabase::new(pool);

        let performer = CreatePerformer {
            name: "Performer".to_string(),
            image_url: Some("image_url".to_string()),
            stash_id: Some("stash_id".to_string()),
            gender: Some(Gender::Female),
        };
        db.insert(performer.clone()).await?;

        // should not be able to insert the same performer twice
        let result = db.insert(performer).await;
        assert!(result.is_err());

        Ok(())
    }

    #[sqlx::test]
    async fn find_all_performers(pool: SqlitePool) -> Result<()> {
        let db = PerformersDatabase::new(pool);

        let performers = db.find_all().await?;
        assert_eq!(performers.len(), 0);

        for i in 0..10 {
            let performer = CreatePerformer {
                name: format!("Performer {}", i),
                image_url: Some("image_url".to_string()),
                stash_id: Some("stash_id".to_string()),
                gender: None,
            };
            db.insert(performer).await?;
        }

        let performers = db.find_all().await?;
        assert_eq!(performers.len(), 10);

        Ok(())
    }
}
