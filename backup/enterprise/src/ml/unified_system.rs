use crate::ml::data_acquisition::MLDataManager;
use crate::ml::enterprise_integration::MLEnterpriseIntegration;
use crate::integration::unified_data_system::UnifiedDataSystem;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Unified ML System that connects data acquisition with enterprise integration
pub struct UnifiedMLSystem {
    // Core Components
    data_manager: Arc<MLDataManager>,
    enterprise_integration: Arc<MLEnterpriseIntegration>,
    unified_data: Arc<UnifiedDataSystem>,
    
    // State Management
    state: Arc<RwLock<MLSystemState>>,
    
    // Metrics
    metrics: Arc<MLMetrics>,
}

#[derive(Debug)]
pub struct MLSystemState {
    knowledge_base_ready: bool,
    data_streams_active: bool,
    prediction_models_loaded: bool,
    enterprise_integration_ready: bool,
}

impl UnifiedMLSystem {
    pub async fn new(
        unified_data: Arc<UnifiedDataSystem>,
    ) -> Result<Self, anyhow::Error> {
        let data_manager = Arc::new(MLDataManager::new().await?);
        let enterprise_integration = Arc::new(MLEnterpriseIntegration::new(unified_data.clone()));
        
        Ok(Self {
            data_manager,
            enterprise_integration,
            unified_data,
            state: Arc::new(RwLock::new(MLSystemState {
                knowledge_base_ready: false,
                data_streams_active: false,
                prediction_models_loaded: false,
                enterprise_integration_ready: false,
            })),
            metrics: Arc::new(MLMetrics::new()),
        })
    }

    pub async fn initialize(&self) -> Result<(), anyhow::Error> {
        // 1. Initialize data acquisition
        self.data_manager.initialize().await?;
        
        // 2. Connect to enterprise integration
        self.connect_enterprise_integration().await?;
        
        // 3. Start data pipelines
        self.start_data_pipelines().await?;
        
        // 4. Initialize prediction models
        self.initialize_prediction_models().await?;
        
        // Update state
        let mut state = self.state.write().await;
        state.knowledge_base_ready = true;
        state.data_streams_active = true;
        state.prediction_models_loaded = true;
        state.enterprise_integration_ready = true;

        Ok(())
    }

    async fn connect_enterprise_integration(&self) -> Result<(), anyhow::Error> {
        // Connect data manager to unified data system
        let knowledge_base = self.data_manager.knowledge_base.read().await;
        
        // Register data handlers
        self.unified_data.register_handler(
            DataType::MLPrediction,
            Box::new(self.create_ml_handler()),
        ).await?;

        Ok(())
    }

    async fn start_data_pipelines(&self) -> Result<(), anyhow::Error> {
        // Create data pipeline from acquisition to enterprise
        let pipeline = DataPipeline::new()
            .add_source(self.data_manager.clone())
            .add_processor(self.create_ml_processor())
            .add_sink(self.enterprise_integration.clone());

        // Start pipeline
        pipeline.start().await?;

        Ok(())
    }

    fn create_ml_handler(&self) -> impl DataHandler {
        Box::new(move |data: UnifiedDataRecord| {
            Box::pin(async move {
                match data.data_type {
                    DataType::MarketData => {
                        // Process market data
                        self.process_market_data(data).await
                    },
                    DataType::Transaction => {
                        // Process transaction data
                        self.process_transaction_data(data).await
                    },
                    _ => Ok(()),
                }
            })
        })
    }

    pub async fn process_market_data(
        &self,
        data: UnifiedDataRecord,
    ) -> Result<(), anyhow::Error> {
        // Update knowledge base
        let mut knowledge_base = self.data_manager.knowledge_base.write().await;
        knowledge_base.integrate_market_data(&data).await?;

        // Generate predictions
        let predictions = self.enterprise_integration
            .process_enterprise_data(&data)
            .await?;

        // Store predictions
        self.unified_data
            .store_predictions(predictions)
            .await?;

        Ok(())
    }

    pub async fn process_transaction_data(
        &self,
        data: UnifiedDataRecord,
    ) -> Result<(), anyhow::Error> {
        // Process with knowledge base
        let knowledge_base = self.data_manager.knowledge_base.read().await;
        let enriched_data = knowledge_base.enrich_transaction_data(&data).await?;

        // Generate insights
        let insights = self.enterprise_integration
            .process_enterprise_data(&enriched_data)
            .await?;

        // Store insights
        self.unified_data
            .store_insights(insights)
            .await?;

        Ok(())
    }

    pub async fn get_system_status(&self) -> MLSystemStatus {
        let state = self.state.read().await;
        MLSystemStatus {
            knowledge_base_ready: state.knowledge_base_ready,
            data_streams_active: state.data_streams_active,
            prediction_models_loaded: state.prediction_models_loaded,
            enterprise_integration_ready: state.enterprise_integration_ready,
            metrics: self.metrics.get_current_metrics().await,
        }
    }
} 