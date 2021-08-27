use crate::WorkerLayer;
use futures_util::future::poll_fn;
use std::{future::Future, time::Duration};
use tokio::time::interval;
use tower_layer::Layer;
use tower_service::Service;

/// Spawns a worker task with a clone of the inner service that periodically
/// makes a request to the inner service.
/// 
/// The default Tokio executor is used to run the given service, which means
/// that this layer can only be used on the Tokio runtime.
///
/// See the module documentation for more details.
pub struct PeriodicLayer<T> {
    make_request: T,
    period: Duration,
}

impl<T> PeriodicLayer<T> {
    /// Creates a new [`PeriodicLayer`] with the provided `make_request` closure
    /// and `period`.
    /// 
    /// `make_request` returns a request to be called on the inner service.
    /// `period` gives with interval with which to send the request from `make_request`.
    pub fn new(make_request: T, period: Duration) -> Self {
        PeriodicLayer {
            make_request,
            period,
        }
    }
}

impl<S, T, F, Request> Layer<S> for PeriodicLayer<T>
where
    S: Service<Request, Future = F> + Clone + Send + 'static,
    T: Fn() -> Request + Clone + Send + Sync + 'static,
    F: Future<Output = Result<S::Response, S::Error>> + Send + 'static,
    Request: Send,
{
    type Service = S;

    fn layer(&self, inner: S) -> Self::Service {
        let make_request = self.make_request.clone();
        let period = self.period;
        let make_worker = |mut service: S| {
            let make_request = make_request.clone();
            let period = period;

            async move {
                let mut interval = interval(period);

                loop {
                    let _ = interval.tick().await;

                    if poll_fn(|cx| service.poll_ready(cx)).await.is_err() {
                        break;
                    };

                    if service.call(make_request()).await.is_err() {
                        break;
                    }
                }
            }
        };
        let worker_layer = WorkerLayer::new(make_worker);

        Layer::<S>::layer(&worker_layer, inner)
    }
}
