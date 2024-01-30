use foundations::telemetry::metrics::{metrics, Counter, Gauge};

#[metrics]
pub(crate) mod metrics {
    /// Number of total bolas arenas created
    pub(crate) fn arenas_total() -> Counter;

    /// Number of total bolas created within all of the arenas
    pub(crate) fn bolas_total() -> Counter;

    /// Number of active bolas arenas
    pub(crate) fn arenas_active() -> Gauge;

    /// Number of active bolas within all active arenas
    pub(crate) fn bolas_active() -> Gauge;
}
