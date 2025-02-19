use crate::wallet::{
    WalletDashboard,
    UserDashboard,
    WalletAlignment,
};

pub struct WalletAlignmentManager {
    wallet_dashboard: Arc<WalletDashboard>,
    user_dashboard: Arc<UserDashboard>,
    wallet_alignment: Arc<WalletAlignment>,
    state_manager: Arc<StateManager>,
}

impl WalletAlignmentManager {
    pub async fn align_wallet_systems(
        &self,
        request: AlignmentRequest,
        context: &AlignmentContext,
    ) -> Result<AlignmentResult, AlignmentError> {
        // 1. Validate Alignment Request
        self.validate_alignment_request(&request, context).await?;
        
        // 2. Process Alignment
        let result = match request.alignment_type {
            AlignmentType::WalletState => {
                self.align_wallet_state(request.data, context).await?
            },
            AlignmentType::UserDashboard => {
                self.align_user_dashboard(request.data, context).await?
            },
            AlignmentType::SystemMetrics => {
                self.align_system_metrics(request.data, context).await?
            },
        };
        
        // 3. Update State
        self.state_manager
            .update_alignment_state(&result)
            .await?;
        
        // 4. Verify Alignment
        self.verify_alignment_result(&result).await?;

        Ok(result)
    }

    async fn align_wallet_state(
        &self,
        data: AlignmentData,
        context: &AlignmentContext,
    ) -> Result<AlignmentResult, AlignmentError> {
        // Align wallet dashboard
        let wallet_result = self.wallet_dashboard
            .process_dashboard_update(data.into(), context)
            .await?;
            
        // Align user dashboard
        let user_result = self.user_dashboard
            .process_user_dashboard(data.into(), context)
            .await?;
            
        // Combine results
        let combined = self.combine_alignment_results(
            wallet_result,
            user_result,
        ).await?;

        Ok(AlignmentResult::WalletState(combined))
    }
} 