use futures::future::Future;
use std::pin::Pin;

pub(crate) type ResultFuture<R, E> = Pin<Box<dyn Future<Output = Result<R, E>> + Send>>;
