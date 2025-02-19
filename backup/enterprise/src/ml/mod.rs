//! Enterprise ML System with Bitcoin-First Focus

mod unified_pipeline;
mod research;
mod monitoring;
mod metrics;

use crate::{
    bitcoin::core::BitcoinCore,
    analytics::AdvancedAnalytics,
    validation::ValidationEngine,
    monitoring::MonitoringSystem,
};
use std::sync::Arc;
use std::time::Duration;
use std::path::PathBuf;
use crate::ml::unified_pipeline::MLError;
use crate::ml::metrics::SystemAnalysis;
use chrono;
use tokio::sync::RwLock;
use crate::transaction::Transaction;

pub use self::{
    unified_pipeline::UnifiedMLPipeline,
    research::BitcoinResearchPipeline,
    monitoring::MonitoringPipeline,
    metrics::MLMetrics,
};

pub mod models;
pub mod features;
pub mod analysis;
pub mod training;

pub struct MLSystem {
    model_manager: Arc<ModelManager>,
    feature_extractor: Arc<FeatureExtractor>,
    pattern_analyzer: Arc<PatternAnalyzer>,
    risk_assessor: Arc<RiskAssessor>,
    training_manager: Arc<TrainingManager>,
}

impl MLSystem {
    pub async fn analyze_transaction(
        &self,
        tx: &Transaction,
        context: &MLContext,
    ) -> Result<MLAnalysis, MLError> {
        // Extract features from transaction
        let features = self.feature_extractor
            .extract_transaction_features(tx)
            .await?;
            
        // Perform risk assessment
        let risk_score = self.risk_assessor
            .assess_transaction_risk(&features, context)
            .await?;
            
        // Analyze patterns
        let patterns = self.pattern_analyzer
            .analyze_transaction_patterns(&features)
            .await?;
            
        Ok(MLAnalysis {
            risk_score,
            patterns,
            features,
        })
    }
}

/// Core ML system configuration
#[derive(Debug, Clone)]
pub struct MLConfig {
    // Bitcoin-specific settings
    pub bitcoin_network: bitcoin::Network,
    pub analysis_interval: Duration,
    pub risk_threshold: f64,
    
    // ML model settings
    pub model_path: PathBuf,
    pub update_interval: Duration,
    pub confidence_threshold: f64,
    
    // Resource limits
    pub max_memory_usage: usize,
    pub max_cpu_usage: f32,
}

/// Enterprise ML system manager
pub struct EnterpriseML {
    // Core components
    bitcoin_core: Arc<BitcoinCore>,
    validation: Arc<ValidationEngine>,
    analytics: Arc<AdvancedAnalytics>,
    monitoring: Arc<MonitoringSystem>,
    
    // ML pipelines
    unified_pipeline: Arc<UnifiedMLPipeline>,
    research_pipeline: Arc<BitcoinResearchPipeline>,
    monitoring_pipeline: Arc<MonitoringPipeline>,
    
    // State and metrics
    config: MLConfig,
    metrics: MLMetrics,
}

impl EnterpriseML {
    pub async fn new(config: MLConfig) -> Result<Self, MLError> {
        // Initialize core components
        let bitcoin_core = Arc::new(BitcoinCore::new(config.bitcoin_network)?);
        let validation = Arc::new(ValidationEngine::new(config.bitcoin_network)?);
        let analytics = Arc::new(AdvancedAnalytics::new()?);
        let monitoring = Arc::new(MonitoringSystem::new()?);

        // Initialize ML pipelines
        let unified_pipeline = Arc::new(UnifiedMLPipeline::new(
            bitcoin_core.clone(),
            analytics.clone(),
            validation.clone(),
        )?);

        let research_pipeline = Arc::new(BitcoinResearchPipeline::new(
            bitcoin_core.clone(),
            analytics.clone(),
        )?);

        let monitoring_pipeline = Arc::new(MonitoringPipeline::new(
            monitoring.clone(),
            analytics.clone(),
        )?);

        Ok(Self {
            bitcoin_core,
            validation,
            analytics,
            monitoring,
            unified_pipeline,
            research_pipeline,
            monitoring_pipeline,
            config,
            metrics: MLMetrics::new(),
        })
    }

    pub async fn analyze_system_state(&self) -> Result<SystemAnalysis, MLError> {
        // Start metrics tracking
        let start = std::time::Instant::now();

        // 1. Get Bitcoin network state
        let network_state = self.bitcoin_core.get_network_state().await?;

        // 2. Run unified analysis
        let unified_analysis = self.unified_pipeline
            .analyze_system_state(&network_state)
            .await?;

        // 3. Run research analysis
        let research_analysis = self.research_pipeline
            .analyze_bitcoin_metrics(&network_state)
            .await?;

        // 4. Run monitoring analysis
        let monitoring_analysis = self.monitoring_pipeline
            .analyze_system_metrics(&network_state)
            .await?;

        // Record metrics
        self.metrics.record_analysis(start.elapsed());

        Ok(SystemAnalysis {
            unified: unified_analysis,
            research: research_analysis,
            monitoring: monitoring_analysis,
            timestamp: chrono::Utc::now(),
        })
    }

    pub async fn update_models(&self) -> Result<(), MLError> {
        // Update ML models with latest data
        self.unified_pipeline.update_models().await?;
        self.research_pipeline.update_models().await?;
        self.monitoring_pipeline.update_models().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_enterprise_ml() {
        let config = MLConfig {
            bitcoin_network: bitcoin::Network::Testnet,
            analysis_interval: Duration::from_secs(60),
            risk_threshold: 0.8,
            model_path: PathBuf::from("models"),
            update_interval: Duration::from_secs(3600),
            confidence_threshold: 0.9,
            max_memory_usage: 1024 * 1024 * 1024, // 1GB
            max_cpu_usage: 0.8,
        };

        let ml = EnterpriseML::new(config).await.unwrap();
        let analysis = ml.analyze_system_state().await.unwrap();

        assert!(analysis.unified.confidence > 0.8);
        assert!(analysis.research.metrics.is_valid());
        assert!(analysis.monitoring.system_health.is_healthy());
    }
}
