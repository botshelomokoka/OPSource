use std::sync::Arc;
use tokio::sync::RwLock;
use metrics::{Counter, Gauge, Histogram};
use tracing::{info, warn, error};

#[derive(Debug, thiserror::Error)]
pub enum RateLimitError {
    #[error("Rate limit exceeded: {0}")]
    LimitExceeded(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Security error: {0}")]
    SecurityError(String),
    #[error("ML analysis error: {0}")]
    MLError(String),
}

pub struct RateLimiter {
    governor: Governor,
    limits: RwLock<RateLimits>,
    metrics: RateLimitMetrics,
    audit_logger: Arc<AuditLogger>,
    policy_engine: Arc<PolicyEngine>,
    security_context: Arc<SecurityContext>,
    ml_analyzer: Arc<MLAnalyzer>,
    compliance_engine: Arc<ComplianceEngine>,
    web5_integration: Option<Arc<Web5Integration>>,
    mode: SystemMode,
}

impl RateLimiter {
    pub async fn new(config: RateLimitConfig) -> Result<Self, RateLimitError> {
        let metrics = RateLimitMetrics::new();
        let audit_logger = Arc::new(AuditLogger::new(config.audit_config)?);
        let policy_engine = Arc::new(PolicyEngine::new(config.policy_rules)?);
        let ml_analyzer = Arc::new(MLAnalyzer::new(config.ml_config)?);
        let compliance_engine = Arc::new(ComplianceEngine::new(config.compliance_config)?);
        
        let web5_integration = if config.mode.supports_web5() {
            Some(Arc::new(Web5Integration::new(config.web5_config?).await?))
        } else {
            None
        };

        Ok(Self {
            governor: Governor::new(config.mode.get_governor_config()?),
            limits: RwLock::new(RateLimits::from_config(config.clone())?),
            metrics,
            audit_logger,
            policy_engine,
            security_context: Arc::new(SecurityContext::new()),
            ml_analyzer,
            compliance_engine,
            web5_integration,
            mode: config.mode,
        })
    }

    pub async fn check_rate_limit(&self, context: &SecurityContext) -> Result<(), RateLimitError> {
        // Record metrics
        self.metrics.rate_limit_check_counter.increment(1);
        let timer = self.metrics.rate_limit_check_duration.start_timer();

        // Check security context
        if let Err(e) = self.security_context.validate(context) {
            error!("Security validation failed: {}", e);
            return Err(RateLimitError::SecurityError(e.to_string()));
        }

        // Apply rate limiting
        let limits = self.limits.read().await;
        if let Err(e) = self.governor.check_rate_limit(&limits, context).await {
            warn!("Rate limit exceeded: {}", e);
            self.metrics.rate_limit_exceeded_counter.increment(1);
            return Err(RateLimitError::LimitExceeded(e.to_string()));
        }

        // ML-based analysis
        if let Err(e) = self.ml_analyzer.analyze_request(context).await {
            error!("ML analysis failed: {}", e);
            return Err(RateLimitError::MLError(e.to_string()));
        }

        // Compliance check
        if let Err(e) = self.compliance_engine.check_compliance(context).await {
            error!("Compliance check failed: {}", e);
            return Err(RateLimitError::SecurityError(e.to_string()));
        }

        // Log audit event
        self.audit_logger.log_rate_limit_check(context).await?;

        // Stop timer and record success
        drop(timer);
        self.metrics.rate_limit_success_counter.increment(1);

        Ok(())
    }
}

struct RateLimitMetrics {
    rate_limit_check_counter: Counter,
    rate_limit_exceeded_counter: Counter,
    rate_limit_success_counter: Counter,
    rate_limit_check_duration: Histogram,
}

impl RateLimitMetrics {
    fn new() -> Self {
        Self {
            rate_limit_check_counter: Counter::new("rate_limit_checks_total"),
            rate_limit_exceeded_counter: Counter::new("rate_limit_exceeded_total"),
            rate_limit_success_counter: Counter::new("rate_limit_success_total"),
            rate_limit_check_duration: Histogram::new("rate_limit_check_duration_seconds"),
        }
    }
}