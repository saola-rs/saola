#[cfg(feature = "telemetry")]
pub use telemetry::TraceParent;

#[cfg(not(feature = "telemetry"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraceParent;

#[cfg(not(feature = "telemetry"))]
impl TraceParent {
    pub fn sampled(&self) -> bool {
        false
    }
}

#[cfg(not(feature = "telemetry"))]
impl std::fmt::Display for TraceParent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
