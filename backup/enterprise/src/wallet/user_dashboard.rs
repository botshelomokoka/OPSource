use crate::web5::Web5Manager;
use crate::ml::MLProcessor;

pub struct UserDashboard {
    web5_manager: Arc<Web5Manager>,
    ml_processor: Arc<MLProcessor>,
    user_preferences: Arc<UserPreferences>,
    dashboard_manager: Arc<DashboardManager>,
}

impl UserDashboard {
    pub async fn process_user_dashboard(
        &self,
        request: UserDashboardRequest,
        context: &UserContext,
    ) -> Result<UserDashboardResult, DashboardError> {
        // 1. Process User Identity
        let identity = self.web5_manager
            .process_identity(&request.user_id)
            .await?;
        
        // 2. Load Preferences
        let preferences = self.user_preferences
            .load_preferences(&identity)
            .await?;
        
        // 3. Process Dashboard
        let result = match request.view_type {
            DashboardViewType::Overview => {
                self.process_overview_view(
                    &identity,
                    &preferences,
                    context,
                ).await?
            },
            DashboardViewType::Transactions => {
                self.process_transaction_view(
                    &identity,
                    &preferences,
                    context,
                ).await?
            },
            DashboardViewType::Assets => {
                self.process_asset_view(
                    &identity,
                    &preferences,
                    context,
                ).await?
            },
            DashboardViewType::Analytics => {
                self.process_analytics_view(
                    &identity,
                    &preferences,
                    context,
                ).await?
            },
        };
        
        // 4. Apply ML Enhancements
        let enhanced_result = self.ml_processor
            .enhance_dashboard_view(result)
            .await?;

        Ok(enhanced_result)
    }

    async fn process_overview_view(
        &self,
        identity: &UserIdentity,
        preferences: &UserPreferences,
        context: &UserContext,
    ) -> Result<DashboardView, DashboardError> {
        // Collect overview data
        let wallet_data = self.collect_wallet_data(identity).await?;
        let asset_data = self.collect_asset_data(identity).await?;
        let tx_data = self.collect_transaction_data(identity).await?;
        
        // Apply preferences
        let customized_view = self.apply_view_preferences(
            wallet_data,
            asset_data,
            tx_data,
            preferences,
        ).await?;

        Ok(customized_view)
    }
} 