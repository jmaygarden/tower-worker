use std::future::Future;
use tower_layer::Layer;

/// Spawns a worker task with a clone of the inner service.
///
/// The default Tokio executor is used to run the given service, which means
/// that this layer can only be used on the Tokio runtime.
///
/// See the module documentation for more details.
pub struct WorkerLayer<T> {
    make_worker: T,
}

impl<T> WorkerLayer<T> {
    /// Creates a new [`WorkerLayer`] with the provided `make_worker` closure.
    ///
    /// `make_worker` accepts a clone of the inner service and returns a
    /// `Future` that will be spawned as a tokio task.
    ///
    /// [`Future`]: std::future::Future
    pub fn new(make_worker: T) -> Self {
        Self { make_worker }
    }
}

impl<S, T, F> Layer<S> for WorkerLayer<T>
where
    S: Clone,
    T: Fn(S) -> F,
    F: Future<Output = ()> + Send + 'static,
{
    type Service = S;

    fn layer(&self, inner: S) -> Self::Service {
        tokio::spawn((self.make_worker)(inner.clone()));

        inner
    }
}
