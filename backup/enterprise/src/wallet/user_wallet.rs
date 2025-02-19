use crate::web5::Web5Manager;
use crate::ml::MLProcessor;

pub struct UserWallet {
    web5_manager: Arc<Web5Manager>,
    ml_processor: Arc<MLProcessor>,
    identity_manager: Arc<IdentityManager>,
    preference_engine: Arc<PreferenceEngine>,
}

impl UserWallet {
    pub async fn process_user_operation(
        &self,
        operation: UserOperation,
        context: &UserContext,
    ) -> Result<UserOperationResult, WalletError> {
        // 1. Process Identity
        let identity = self.process_user_identity(&operation, context).await?;
        
        // 2. Apply User Preferences
        let preferences = self.apply_user_preferences(
            &operation,
            &identity,
            context,
        ).await?;
        
        // 3. ML Enhancement
        let enhanced_operation = self.enhance_with_ml(
            operation,
            &preferences,
            context,
        ).await?;
        
        // 4. Process Operation
        let result = self.process_enhanced_operation(
            enhanced_operation,
            &identity,
            context,
        ).await?;

        // 5. Update User Profile
        self.update_user_profile(
            &identity,
            &result,
            context,
        ).await?;

        Ok(result)
    }

    async fn process_user_identity(
        &self,
        operation: &UserOperation,
        context: &UserContext,
    ) -> Result<UserIdentity, WalletError> {
        // Handle Web5 DID operations
        let did_result = self.web5_manager
            .process_identity_operation(operation.into())
            .await?;
            
        // Update identity credentials
        let credentials = self.identity_manager
            .update_credentials(did_result)
            .await?;
            
        Ok(UserIdentity {
            did: did_result.did,
            credentials,
            preferences: self.get_user_preferences(context).await?,
        })
    }
} 