use tracing_core::Subscriber;
use tracing_subscriber::layer::Layer;
use tracing_subscriber::registry::LookupSpan;

pub struct OsLogger;

impl OsLogger {
    pub fn new<S, C>(_subsystem: S, _category: C) -> Self
    where
        S: AsRef<str>,
        C: AsRef<str>,
    {
        Self
    }
}

impl Default for OsLogger {
    fn default() -> Self {
        Self
    }
}

impl<S> Layer<S> for OsLogger where S: Subscriber + for<'a> LookupSpan<'a> {}
