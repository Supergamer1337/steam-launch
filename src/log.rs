use anyhow::Result;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn enable() -> Result<()> {
    #[cfg(not(debug_assertions))]
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    #[cfg(debug_assertions)]
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .with_writer(std::io::stderr)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber.");

    Ok(())
}
