use crate::business::{
    ml_engine::BusinessMLEngine,
    revenue::RevenueAnalyzer,
    market_intelligence::MarketIntelligence,
};
use crate::ml::unified_system::UnifiedMLSystem;
use crate::enterprise::risk::RiskManagementSystem;

/// Business API Gateway
pub struct BusinessAPIGateway {
    ml_engine: Arc<BusinessMLEngine>,
    revenue_analyzer: Arc<RevenueAnalyzer>,
    market_intelligence: Arc<MarketIntelligence>,
    risk_system: Arc<RiskManagementSystem>,
    unified_ml: Arc<UnifiedMLSystem>,
}

impl BusinessAPIGateway {
    pub async fn new(
        unified_ml: Arc<UnifiedMLSystem>,
        risk_system: Arc<RiskManagementSystem>,
    ) -> Result<Self, BusinessError> {
        let ml_engine = Arc::new(BusinessMLEngine::new(unified_ml.clone()));
        let revenue_analyzer = Arc::new(RevenueAnalyzer::new(
            unified_ml.clone(),
            risk_system.clone(),
        ));
        let market_intelligence = Arc::new(MarketIntelligence::new(
            unified_ml.clone(),
            risk_system.clone(),
        ));

        Ok(Self {
            ml_engine,
            revenue_analyzer,
            market_intelligence,
            risk_system,
            unified_ml,
        })
    }

    /// Analyze Business Opportunity
    pub async fn analyze_opportunity(
        &self,
        context: BusinessContext,
    ) -> Result<BusinessOpportunity, BusinessError> {
        // 1. Market Analysis
        let market_analysis = self.market_intelligence
            .analyze_market_conditions(&context)
            .await?;

        // 2. Risk Assessment
        let risk_profile = self.risk_system
            .evaluate_risk_profile(&context.portfolio)
            .await?;

        // 3. Revenue Analysis
        let revenue_potential = self.revenue_analyzer
            .calculate_potential(
                &market_analysis,
                &risk_profile,
                &context,
            )
            .await?;

        // 4. ML-based Recommendations
        let recommendations = self.ml_engine
            .analyze_business_opportunity(&context)
            .await?;

        Ok(BusinessOpportunity {
            market_analysis,
            risk_profile,
            revenue_potential,
            recommendations,
            timestamp: chrono::Utc::now(),
        })
    }

    /// Portfolio Management API
    pub async fn optimize_portfolio(
        &self,
        portfolio: Portfolio,
        constraints: PortfolioConstraints,
    ) -> Result<PortfolioOptimization, BusinessError> {
        // 1. Current Portfolio Analysis
        let portfolio_analysis = self.analyze_portfolio(&portfolio).await?;

        // 2. Market Conditions
        let market_conditions = self.market_intelligence
            .get_current_conditions()
            .await?;

        // 3. Risk Constraints
        let risk_assessment = self.risk_system
            .assess_portfolio_risk(&portfolio, &constraints)
            .await?;

        // 4. ML Optimization
        let optimization = self.ml_engine
            .optimize_portfolio(
                &portfolio_analysis,
                &market_conditions,
                &risk_assessment,
                &constraints,
            )
            .await?;

        Ok(optimization)
    }

    /// Revenue Optimization API
    pub async fn optimize_revenue_streams(
        &self,
        current_revenue: RevenueData,
        optimization_params: OptimizationParams,
    ) -> Result<RevenueOptimization, BusinessError> {
        // 1. Revenue Analysis
        let revenue_analysis = self.revenue_analyzer
            .analyze_current_revenue(&current_revenue)
            .await?;

        // 2. Market Impact
        let market_impact = self.market_intelligence
            .assess_revenue_impact(&revenue_analysis)
            .await?;

        // 3. Risk Assessment
        let risk_assessment = self.risk_system
            .assess_revenue_risk(&revenue_analysis)
            .await?;

        // 4. ML-based Optimization
        let optimization = self.ml_engine
            .optimize_revenue(
                &revenue_analysis,
                &market_impact,
                &risk_assessment,
                &optimization_params,
            )
            .await?;

        Ok(optimization)
    }

    /// Risk Management API
    pub async fn manage_business_risk(
        &self,
        risk_params: RiskManagementParams,
    ) -> Result<RiskManagementStrategy, BusinessError> {
        // 1. Current Risk Analysis
        let current_risk = self.risk_system
            .analyze_current_risk(&risk_params)
            .await?;

        // 2. Market Risk Factors
        let market_risk = self.market_intelligence
            .assess_market_risk()
            .await?;

        // 3. ML Risk Predictions
        let risk_predictions = self.ml_engine
            .predict_risk_factors(&current_risk, &market_risk)
            .await?;

        // 4. Risk Strategy Generation
        let strategy = self.generate_risk_strategy(
            &current_risk,
            &market_risk,
            &risk_predictions,
            &risk_params,
        ).await?;

        Ok(strategy)
    }

    /// Market Intelligence API
    pub async fn get_market_insights(
        &self,
        params: MarketInsightParams,
    ) -> Result<MarketInsights, BusinessError> {
        // 1. Market Analysis
        let market_analysis = self.market_intelligence
            .analyze_market_data(&params)
            .await?;

        // 2. ML Insights
        let ml_insights = self.ml_engine
            .analyze_market_patterns(&market_analysis)
            .await?;

        // 3. Risk Impact
        let risk_impact = self.risk_system
            .assess_market_risk_impact(&market_analysis)
            .await?;

        Ok(MarketInsights {
            market_analysis,
            ml_insights,
            risk_impact,
            timestamp: chrono::Utc::now(),
        })
    }

    /// Internal helper methods
    async fn analyze_portfolio(
        &self,
        portfolio: &Portfolio,
    ) -> Result<PortfolioAnalysis, BusinessError> {
        let analysis = self.ml_engine
            .analyze_portfolio_composition(portfolio)
            .await?;

        Ok(analysis)
    }

    async fn generate_risk_strategy(
        &self,
        current_risk: &RiskAnalysis,
        market_risk: &MarketRisk,
        predictions: &RiskPredictions,
        params: &RiskManagementParams,
    ) -> Result<RiskManagementStrategy, BusinessError> {
        let strategy = RiskManagementStrategy {
            risk_mitigation: self.generate_mitigation_steps(
                current_risk,
                market_risk,
                predictions,
            )?,
            monitoring_plan: self.create_monitoring_plan(params)?,
            contingency_plans: self.generate_contingency_plans(
                current_risk,
                predictions,
            )?,
            review_schedule: self.create_review_schedule(params)?,
        };

        Ok(strategy)
    }
} 