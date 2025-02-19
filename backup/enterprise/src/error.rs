use anya_core::error::CoreError;
use thiserror::Error;
use serde::{Serialize, Deserialize};
use bitcoin::Error as BitcoinError;
use lightning::Error as LightningError;

#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum EnterpriseError {
    #[error("Core error: {0}")]
    Core(#[from] CoreError),

    #[error("Rate limit error: {kind} - {message}")]
    RateLimit {
        kind: RateLimitErrorKind,
        message: String,
        context: RateLimitContext,
    },

    #[error("Mode error: {kind} - {message}")]
    Mode {
        kind: ModeErrorKind,
        message: String,
        mode: SystemMode,
        attempted_operation: String,
    },

    #[error("Scaling error: {kind} - {message}")]
    Scaling {
        kind: ScalingErrorKind,
        message: String,
        context: ScalingContext,
    },

    #[error("Feature error: {feature} not available in {mode} mode")]
    FeatureUnavailable {
        feature: String,
        mode: SystemMode,
        required_mode: SystemMode,
    },

    #[error("Bitcoin core error: {0}")]
    BitcoinCore(#[from] BitcoinError),

    #[error("Lightning error: {0}")]
    Lightning(#[from] LightningError),

    #[error("RGB error: {0}")]
    RGB(String),

    #[error("DLC error: {0}")]
    DLC(String),

    #[error("Institutional error: {0}")]
    Institutional(String),

    #[error("Security violation: {0}")]
    SecurityViolation(String),

    #[error("Policy violation: {0}")]
    PolicyViolation(String),

    #[error("Compliance error: {0}")]
    ComplianceError(String),

    #[error("ML system error: {0}")]
    MLSystem(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Agent error: {0}")]
    Agent(String),

    #[error("Protocol error: {kind} - {message}")]
    Protocol {
        kind: String,
        message: String,
    },

    #[error("State error: {0}")]
    State(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl EnterpriseError {
    pub fn handle_error(&self) -> Result<(), Self> {
        match self {
            Self::Core(core_err) if core_err.is_recoverable() => {
                // Attempt recovery
                Ok(())
            },
            Self::RateLimit { kind, .. } if kind.can_retry() => {
                // Schedule retry
                Ok(())
            },
            Self::Scaling { kind, .. } if kind.can_fallback() => {
                // Use fallback configuration
                Ok(())
            },
            _ => Err(self.clone()),
        }
    }
}

// Add logging middleware
pub fn setup_logging() -> Result<(), EnterpriseError> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .init();
    Ok(())
} 