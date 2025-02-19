use tracing::{info, warn, error, Level};
use tracing_subscriber::{FmtSubscriber, EnvFilter};

pub struct LoggingSystem {
    filter: EnvFilter,
}

impl LoggingSystem {
    pub fn init() -> Result<(), EnterpriseError> {
        let subscriber = FmtSubscriber::builder()
            .with_env_filter(EnvFilter::from_default_env()
                .add_directive(Level::INFO.into())
                .add_directive("anya_enterprise=debug".parse()?)
                .add_directive("bitcoin=info".parse()?))
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_file(true)
            .with_line_number(true)
            .with_target(false)
            .pretty()
            .try_init()?;

        Ok(())
    }

    pub fn log_transaction_event(
        &self,
        event: &TransactionEvent,
        context: &EnterpriseContext,
    ) {
        info!(
            target: "transactions",
            event_type = %event.event_type,
            tx_id = %event.tx_id,
            amount = %event.amount,
            "Transaction event occurred"
        );
    }

    pub fn log_security_event(
        &self,
        event: &SecurityEvent,
        context: &SecurityContext,
    ) {
        warn!(
            target: "security",
            event_type = %event.event_type,
            severity = %event.severity,
            "Security event detected"
        );
    }
} 