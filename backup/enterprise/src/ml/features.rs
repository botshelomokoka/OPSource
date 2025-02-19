use bitcoin::Transaction;
use ndarray::{Array1, Array2};

pub struct FeatureExtractor {
    temporal_extractor: TemporalFeatureExtractor,
    graph_extractor: GraphFeatureExtractor,
    behavioral_extractor: BehavioralFeatureExtractor,
}

impl FeatureExtractor {
    pub async fn extract_transaction_features(
        &self,
        tx: &Transaction,
    ) -> Result<TransactionFeatures, MLError> {
        // Extract temporal features
        let temporal_features = self.temporal_extractor
            .extract_features(tx)
            .await?;
            
        // Extract graph-based features
        let graph_features = self.graph_extractor
            .extract_features(tx)
            .await?;
            
        // Extract behavioral features
        let behavioral_features = self.behavioral_extractor
            .extract_features(tx)
            .await?;
            
        Ok(TransactionFeatures {
            temporal: temporal_features,
            graph: graph_features,
            behavioral: behavioral_features,
        })
    }
} 