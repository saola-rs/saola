#[cfg(feature = "metrics")]
pub use prisma_metrics::{
    PRISMA_DATASOURCE_QUERIES_DURATION_HISTOGRAM_MS, PRISMA_DATASOURCE_QUERIES_TOTAL, PRISMA_CLIENT_QUERIES_ACTIVE,
    counter, histogram, guards::GaugeGuard
};

#[cfg(not(feature = "metrics"))]
pub const PRISMA_DATASOURCE_QUERIES_DURATION_HISTOGRAM_MS: &str = "prisma_datasource_queries_duration_histogram_ms";
#[cfg(not(feature = "metrics"))]
pub const PRISMA_DATASOURCE_QUERIES_TOTAL: &str = "prisma_datasource_queries_total";
#[cfg(not(feature = "metrics"))]
pub const PRISMA_CLIENT_QUERIES_ACTIVE: &str = "prisma_client_queries_active";

#[cfg(not(feature = "metrics"))]
#[macro_export]
macro_rules! counter {
    ($name:expr) => {
        {
            let _ = $name;
            $crate::metrics::Dummy::default()
        }
    };
}

#[cfg(not(feature = "metrics"))]
#[macro_export]
macro_rules! histogram {
    ($name:expr) => {
        {
            let _ = $name;
            $crate::metrics::Dummy::default()
        }
    };
}

#[cfg(not(feature = "metrics"))]
#[derive(Default)]
pub struct Dummy;

#[cfg(not(feature = "metrics"))]
impl Dummy {
    pub fn increment(&self, _: u64) {}
    pub fn record(&self, _: f64) {}
}

#[cfg(not(feature = "metrics"))]
#[derive(Default)]
pub struct GaugeGuard;

#[cfg(not(feature = "metrics"))]
impl GaugeGuard {
    pub fn increment(_: &str) -> Self {
        Self
    }
    pub fn decrement(&self) {}
}
