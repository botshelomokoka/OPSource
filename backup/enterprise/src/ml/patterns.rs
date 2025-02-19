use graph_algorithms::GraphAnalyzer;
use clustering::ClusterAnalyzer;

pub struct PatternAnalyzer {
    graph_analyzer: GraphAnalyzer,
    cluster_analyzer: ClusterAnalyzer,
    sequence_analyzer: SequenceAnalyzer,
}

impl PatternAnalyzer {
    pub async fn analyze_transaction_patterns(
        &self,
        features: &TransactionFeatures,
    ) -> Result<PatternAnalysis, MLError> {
        // Analyze graph patterns
        let graph_patterns = self.graph_analyzer
            .analyze_patterns(&features.graph)
            .await?;
            
        // Analyze clustering patterns
        let cluster_patterns = self.cluster_analyzer
            .analyze_patterns(&features.behavioral)
            .await?;
            
        // Analyze sequence patterns
        let sequence_patterns = self.sequence_analyzer
            .analyze_patterns(&features.temporal)
            .await?;
            
        Ok(PatternAnalysis {
            graph_patterns,
            cluster_patterns,
            sequence_patterns,
        })
    }
} 