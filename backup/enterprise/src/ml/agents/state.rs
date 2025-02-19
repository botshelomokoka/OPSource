pub struct MLAgentState {
    market_state: MarketState,
    trading_state: TradingState,
    risk_state: RiskState,
    pattern_state: PatternState,
}

impl MLAgentState {
    pub async fn update_state(
        &mut self,
        analysis: &MLAnalysis,
        context: &AgentContext,
    ) -> Result<(), StateError> {
        // Update market state
        self.market_state.update(&analysis.market_analysis).await?;
        
        // Update trading state
        self.trading_state.update(&analysis.trading_analysis).await?;
        
        // Update risk state
        self.risk_state.update(&analysis.risk_assessment).await?;
        
        // Update pattern state
        self.pattern_state.update(&analysis.patterns).await?;
        
        Ok(())
    }
} 