use crate::ml::unified_system::UnifiedMLSystem;
use crate::enterprise::risk::RiskManagementSystem;

pub struct BusinessMLEngine {
    unified_ml: Arc<UnifiedMLSystem>,
    risk_system: Arc<RiskManagementSystem>,
    revenue_analyzer: Arc<RevenueAnalyzer>,
    market_intelligence: Arc<MarketIntelligence>,
    institutional_patterns: Arc<InstitutionalPatterns>,
}

impl BusinessMLEngine {
    pub async fn analyze_business_opportunity(
        &self,
        context: &BusinessContext,
    ) -> Result<BusinessOpportunity, BusinessError> {
        // 1. Market Analysis
        let market_analysis = self.market_intelligence
            .analyze_market_conditions(context)
            .await?;

        // 2. Risk Assessment
        let risk_profile = self.risk_system
            .evaluate_risk_profile(&context.portfolio)
            .await?;

        // 3. Revenue Potential
        let revenue_potential = self.revenue_analyzer
            .calculate_potential(
                &market_analysis,
                &risk_profile,
                context,
            )
            .await?;

        // 4. Institutional Pattern Analysis
        let pattern_insights = self.institutional_patterns
            .analyze_patterns(&context.institutional_data)
            .await?;

        // 5. Generate Business Recommendations
        let recommendations = self.generate_recommendations(
            &market_analysis,
            &risk_profile,
            &revenue_potential,
            &pattern_insights,
        ).await?;

        Ok(BusinessOpportunity {
            market_analysis,
            risk_profile,
            revenue_potential,
            pattern_insights,
            recommendations,
        })
    }

    async fn generate_recommendations(
        &self,
        market: &MarketAnalysis,
        risk: &RiskProfile,
        revenue: &RevenuePotential,
        patterns: &PatternInsights,
    ) -> Result<BusinessRecommendations, BusinessError> {
        let mut recommendations = BusinessRecommendations::new();

        // Market-based recommendations
        if market.trend.is_bullish() && risk.within_limits() {
            recommendations.add_opportunity(
                OpportunityType::MarketEntry,
                market.confidence_level,
            );
        }

        // Risk-based adjustments
        if risk.exposure_level > risk.threshold {
            recommendations.add_risk_mitigation(
                RiskMitigationType::ReduceExposure,
                risk.exposure_delta,
            );
        }

        // Revenue optimization
        if revenue.potential > revenue.current {
            recommendations.add_revenue_optimization(
                OptimizationType::FeeStructure,
                revenue.optimization_potential,
            );
        }

        // Institutional patterns
        for pattern in patterns.significant_patterns() {
            recommendations.add_institutional_strategy(
                pattern.strategy_type,
                pattern.confidence_level,
            );
        }

        Ok(recommendations)
    }
} 