use futures::Future;
use lazy_static::lazy_static;

use crate::Result;

pub mod estimator;
pub mod log;
pub mod random;
pub mod sentry;
pub mod util;

lazy_static! {
    static ref PARALLELISM: usize = {
        let cpus = num_cpus::get();
        cpus / 2
    };
}

pub async fn parallelize<T, I, F>(futures: I) -> Vec<T>
where
    F: Future<Output = T> + Send,
    I: IntoIterator<Item = F>,
{
    use futures::stream;
    use futures::stream::StreamExt;

    let mut stream = stream::iter(futures).buffer_unordered(*PARALLELISM);

    let mut results = vec![];
    while let Some(result) = stream.next().await {
        results.push(result)
    }

    results
}

#[allow(unused)]
pub async fn try_parallelize<T, I, F>(futures: I) -> Result<Vec<T>>
where
    F: Future<Output = Result<T>> + Send,
    I: IntoIterator<Item = F>,
{
    use futures::stream;
    use futures::stream::StreamExt;

    let mut stream = stream::iter(futures).buffer_unordered(*PARALLELISM);

    let mut results = vec![];
    while let Some(result) = stream.next().await {
        results.push(result?)
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::eyre;
    use itertools::Itertools;
    use sqlx::SqlitePool;

    use crate::data::database::Database;
    use crate::helpers::try_parallelize;

    #[tokio::test]
    async fn test_try_parallelize() {
        let futures = vec![
            futures::future::ready(Ok(1)),
            futures::future::ready(Ok(2)),
            futures::future::ready(Ok(3)),
            futures::future::ready(Err(eyre!("error"))),
        ];

        let results = try_parallelize(futures).await;
        assert!(results.is_err());
    }

    #[sqlx::test]
    async fn test_try_parallelize_with_db(pool: SqlitePool) {
        let database = Database::with_pool(pool);

        let futures = (0..10).map(|id| database.music.get_song(id)).collect_vec();
        let results = try_parallelize(futures).await;
        assert!(results.is_err());
    }
}
