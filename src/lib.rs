#![doc(html_root_url = "https://docs.rs/tower-worker/0.1.0")]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Provides Tower layers focues on wrapping services with asynchronous worker
//! tasks that may also make requests to the wrapped service.

#[cfg(feature = "periodic")]
#[cfg_attr(docsrs, doc(cfg(feature = "periodic")))]
mod periodic;

mod worker;

/// Layer that spawns a worker task that periodically sends a request to the
/// service at a given interval and then passes through the wrapped service..
#[cfg(feature = "periodic")]
#[cfg_attr(docsrs, doc(cfg(feature = "periodic")))]
pub use periodic::PeriodicLayer;

/// Layer that spawns a provided worker task using a closure that accepts a
/// clone of the service and then passes through the wrapped service.
pub use worker::WorkerLayer;
