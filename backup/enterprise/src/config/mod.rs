#[derive(Debug, Clone)]
pub struct UnifiedConfig {
    // Core Configuration
    pub core_config: anya_core::Config,
    
    // Enterprise Configuration
    pub enterprise_config: EnterpriseConfig,
    
    // ML Configuration
    pub ml_config: MLConfig,
    
    // Feature Configuration
    pub feature_config: FeatureConfig,
    
    // System Mode
    pub mode: SystemMode,
}

impl UnifiedConfig {
    pub fn new_from_env() -> Result<Self, ConfigError> {
        // Load configuration from environment
        let core_config = anya_core::Config::from_env()?;
        let enterprise_config = EnterpriseConfig::from_env()?;
        let ml_config = MLConfig::from_env()?;
        let feature_config = FeatureConfig::from_env()?;
        
        Ok(Self {
            core_config,
            enterprise_config,
            ml_config,
            feature_config,
            mode: SystemMode::from_env()?,
        })
    }
} 