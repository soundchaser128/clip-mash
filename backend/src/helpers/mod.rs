use futures::Future;
use lazy_static::lazy_static;

use crate::Result;

pub mod estimator;
pub mod util;

lazy_static! {
    static ref PARALELISM: usize = {
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

    let mut stream = stream::iter(futures).buffer_unordered(*PARALELISM);

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

    let mut stream = stream::iter(futures).buffer_unordered(*PARALELISM);

    let mut results = vec![];
    while let Some(result) = stream.next().await {
        results.push(result?)
    }

    Ok(results)
}
