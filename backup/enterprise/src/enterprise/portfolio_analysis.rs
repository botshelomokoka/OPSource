use crate::ml::unified_system::UnifiedMLSystem;
use crate::business::market_intelligence::MarketIntelligence;
use crate::enterprise::risk::RiskManagementSystem;

pub struct PortfolioAnalyzer {
    ml_system: Arc<UnifiedMLSystem>,
    market_intelligence: Arc<MarketIntelligence>,
    risk_system: Arc<RiskManagementSystem>,
    metrics: PortfolioMetrics,
}

impl PortfolioAnalyzer {
    pub async fn analyze_current_portfolio(
        &self,
        market: &MarketAnalysis,
    ) -> Result<PortfolioAnalysis, AgentError> {
        let start = Instant::now();

        // 1. Asset Allocation Analysis
        let allocation_analysis = self.analyze_asset_allocation(market).await?;

        // 2. Risk Exposure Analysis
        let risk_exposure = self.analyze_risk_exposure(&allocation_analysis).await?;

        // 3. Performance Analysis
        let performance = self.analyze_performance_metrics(
            &allocation_analysis,
            &risk_exposure,
        ).await?;

        // 4. ML-based Pattern Analysis
        let patterns = self.ml_system
            .analyze_portfolio_patterns(&allocation_analysis)
            .await?;

        // 5. Market Correlation Analysis
        let correlations = self.analyze_market_correlations(
            &allocation_analysis,
            market,
        ).await?;

        self.metrics.record_analysis(start.elapsed());

        Ok(PortfolioAnalysis {
            allocation: allocation_analysis,
            risk_exposure,
            performance,
            patterns,
            correlations,
            timestamp: chrono::Utc::now(),
        })
    }

    async fn analyze_asset_allocation(
        &self,
        market: &MarketAnalysis,
    ) -> Result<AssetAllocation, AgentError> {
        let allocations = AssetAllocation {
            asset_distribution: self.calculate_asset_distribution()?,
            sector_exposure: self.calculate_sector_exposure()?,
            geographic_distribution: self.calculate_geographic_distribution()?,
            currency_exposure: self.calculate_currency_exposure()?,
            liquidity_profile: self.calculate_liquidity_profile()?,
        };

        Ok(allocations)
    }

    async fn analyze_risk_exposure(
        &self,
        allocation: &AssetAllocation,
    ) -> Result<RiskExposure, AgentError> {
        let exposure = RiskExposure {
            market_risk: self.calculate_market_risk(allocation)?,
            credit_risk: self.calculate_credit_risk(allocation)?,
            liquidity_risk: self.calculate_liquidity_risk(allocation)?,
            operational_risk: self.calculate_operational_risk()?,
            concentration_risk: self.calculate_concentration_risk(allocation)?,
        };

        Ok(exposure)
    }
} 