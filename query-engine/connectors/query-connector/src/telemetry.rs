pub use query_structure::telemetry::TraceParent;

#[cfg(feature = "telemetry")]
pub use tracing_futures::WithSubscriber;

#[cfg(not(feature = "telemetry"))]
pub trait WithSubscriber: Sized {
    fn with_current_subscriber(self) -> Self {
        self
    }
}

#[cfg(not(feature = "telemetry"))]
impl<T> WithSubscriber for T {}
