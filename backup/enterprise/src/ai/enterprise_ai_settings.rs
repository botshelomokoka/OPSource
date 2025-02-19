use crate::security::SecurityManager;
use crate::monitoring::AIMonitor;
use anya_core::ai::AISettingsManager;

pub struct EnterpriseAISettings {
    ai_settings_manager: Arc<AISettingsManager>,
    security_manager: Arc<SecurityManager>,
    ai_monitor: Arc<AIMonitor>,
    policy_engine: Arc<PolicyEngine>,
}

impl EnterpriseAISettings {
    pub async fn process_enterprise_ai_settings(
        &self,
        settings: EnterpriseAISettings,
        context: &SecurityContext,
    ) -> Result<AISettingsResult, AIError> {
        // 1. Security Validation
        self.security_manager
            .validate_ai_settings(&settings, context)
            .await?;
        
        // 2. Apply Enterprise Policies
        let policy_validated = self.apply_enterprise_policies(
            settings,
            context,
        ).await?;
        
        // 3. Process Settings
        let result = self.ai_settings_manager
            .process_ai_settings(policy_validated, &context.into())
            .await?;
        
        // 4. Monitor Application
        self.ai_monitor
            .track_settings_application(&result)
            .await?;

        Ok(result)
    }

    async fn apply_enterprise_policies(
        &self,
        settings: EnterpriseAISettings,
        context: &SecurityContext,
    ) -> Result<AISettings, AIError> {
        // Apply enterprise policies to settings
        let policy_result = self.policy_engine
            .apply_ai_policies(settings, context)
            .await?;
            
        // Validate compliance
        self.validate_policy_compliance(&policy_result, context).await?;

        Ok(policy_result)
    }
} 