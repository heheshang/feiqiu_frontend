// Logging initialization using tracing framework
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

/// Initialize the tracing logging system
///
/// This function sets up the global tracing subscriber with:
/// - Log level from RUST_LOG environment variable (defaults to "info")
/// - Formatted output with timestamp, level, module path, and message
/// - Support for filtering by module/crate
///
/// # Example
///
/// ```no_run
/// fn main() {
///     neolan_lib::utils::logger::init_logger();
///     tracing::info!("Application started");
/// }
/// ```
///
/// # Environment Variables
///
/// - `RUST_LOG`: Set the default log level (e.g., "debug", "info", "warn", "error")
/// - `RUST_LOG=neolan=debug`: Set debug level for neolan crate only
/// - `RUST_LOG=info,neolan::network=debug`: Mix of default and specific levels
pub fn init_logger() {
    // Read log level from environment variable, default to "info"
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    // Configure the subscriber with formatting
    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            fmt::layer()
                .with_target(true)      // Show module path
                .with_thread_ids(false)  // Don't show thread IDs (not needed for single-threaded)
                .with_level(true)       // Show log level
                .with_thread_names(false)
                .with_file(true)        // Show source file
                .with_line_number(true) // Show line number
        )
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_init() {
        // This test ensures logger can be initialized without panicking
        init_logger();
        tracing::info!("Logger initialized successfully");
    }
}
