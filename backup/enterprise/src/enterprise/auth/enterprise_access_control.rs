pub enum EnterpriseAccessLevel {
    Full,
    Semi,
    Risky,
}

impl EnterpriseAccessLevel {
    pub fn permissions(&self, wallet_supports_rsk: bool, wallet_supports_rollups: bool) -> EnterprisePermissions {
        match self {
            EnterpriseAccessLevel::Full => EnterprisePermissions {
                can_access_sensitive_data: true,
                can_modify_enterprise_settings: true,
                can_use_rsk: true,
                can_use_rollups: true,
                // ... other permissions set accordingly ...
            },
            EnterpriseAccessLevel::Semi => EnterprisePermissions {
                can_access_sensitive_data: true,
                can_modify_enterprise_settings: false,
                can_use_rsk: wallet_supports_rsk,
                can_use_rollups: wallet_supports_rollups,
                // ... other permissions set accordingly ...
            },
            EnterpriseAccessLevel::Risky => EnterprisePermissions {
                can_access_sensitive_data: false,
                can_modify_enterprise_settings: false,
                can_use_rsk: false,
                can_use_rollups: false,
                // ... other permissions set accordingly ...
            },
        }
    }
}

pub struct EnterprisePermissions {
    pub can_access_sensitive_data: bool,
    pub can_modify_enterprise_settings: bool,
    pub can_use_rsk: bool,
    pub can_use_rollups: bool,
    // ... other permission fields ...
}

impl Default for EnterprisePermissions {
    fn default() -> Self {
        Self {
            can_access_sensitive_data: true,
            can_modify_enterprise_settings: true,
            can_use_rsk: true,
            can_use_rollups: true,
            // ... default full permissions ...
        }
    }
} 