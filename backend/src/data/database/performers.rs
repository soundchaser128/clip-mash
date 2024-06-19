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
    pub marker_count: i64,
    pub video_count: i64,
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
        sqlx::query_as!(
            DbPerformer,
            r#"SELECT p.id AS "id!", p.name, p.created_on, p.image_url, p.stash_id, 
                      p.gender AS "gender: Gender", count(DISTINCT vp.video_id) AS "video_count!",
                      count(DISTINCT m.rowid) AS "marker_count!"
               FROM performers p
               LEFT JOIN video_performers vp ON p.id = vp.performer_id
               LEFT JOIN markers m ON m.video_id = vp.video_id
               GROUP BY p.name
               ORDER BY count(DISTINCT m.rowid) DESC"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(From::from)
    }

    pub async fn insert(&self, performer: CreatePerformer) -> Result<i64> {
        info!("Inserting performer: {:?}", performer);
        let result = sqlx::query!(
            "INSERT INTO performers (name, image_url, stash_id, gender, created_on) 
            VALUES ($1, $2, $3, $4, strftime('%s', 'now'))
            ON CONFLICT DO UPDATE SET name = name
            RETURNING id",
            performer.name,
            performer.image_url,
            performer.stash_id,
            performer.gender,
        )
        .fetch_one(&self.pool)
        .await?;

        info!("Inserted performer with id: {:?}", result.id);

        Ok(result.id)
    }

    pub async fn insert_for_video(
        &self,
        performers: &[CreatePerformer],
        video_id: &str,
    ) -> Result<()> {
        for performer in performers {
            let performer_id = self.insert(performer.clone()).await?;
            sqlx::query!(
                "INSERT INTO video_performers (video_id, performer_id) VALUES ($1, $2)",
                video_id,
                performer_id,
            )
            .execute(&self.pool)
            .await?;
        }
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

        // should be able to insert the same performer twice, just ignore the second insert
        db.insert(performer).await?;

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
