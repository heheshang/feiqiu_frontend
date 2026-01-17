// Logging initialization using tracing framework
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

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
///     feiqiu::utils::logger::init_logger();
///     tracing::info!("Application started");
/// }
/// ```
///
/// # Environment Variables
///
/// - `RUST_LOG`: Set the default log level (e.g., "debug", "info", "warn", "error")
/// - `RUST_LOG=feiqiu=debug`: Set debug level for feiqiu crate only
/// - `RUST_LOG=info,feiqiu::network=debug`: Mix of default and specific levels
pub fn init_logger() {
    // Read log level from environment variable, default to "info"
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // Configure the subscriber with formatting
    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            fmt::layer()
                .with_target(true) // Show module path
                .with_thread_ids(false) // Don't show thread IDs (not needed for single-threaded)
                .with_level(true) // Show log level
                .with_thread_names(false)
                .with_file(true) // Show source file
                .with_line_number(true), // Show line number
        )
        .init();
}
