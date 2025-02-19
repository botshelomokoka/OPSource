use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use sysinfo::{System, SystemExt, CpuExt};

/// System Tier Levels
#[derive(Debug, Clone, PartialEq)]
pub enum SystemTier {
    Minimal,    // Basic Bitcoin operations only
    Standard,   // + Basic institutional features
    Advanced,   // + ML core & advanced features
    Enterprise, // Full system with all features
}

/// System Requirements per Tier
#[derive(Debug, Clone)]
pub struct TierRequirements {
    min_cpu_cores: u32,
    min_memory_gb: u64,
    min_storage_gb: u64,
    needs_gpu: bool,
    needs_database: bool,
}

/// System Resource Status
#[derive(Debug)]
pub struct SystemResources {
    cpu_cores: u32,
    memory_gb: u64,
    storage_gb: u64,
    has_gpu: bool,
    has_database: bool,
}

/// Tiered System Manager
pub struct TieredSystem {
    current_tier: Arc<RwLock<SystemTier>>,
    resources: Arc<RwLock<SystemResources>>,
    requirements: HashMap<SystemTier, TierRequirements>,
}

impl TieredSystem {
    pub fn new() -> Self {
        let requirements = HashMap::from([
            (SystemTier::Minimal, TierRequirements {
                min_cpu_cores: 2,
                min_memory_gb: 4,
                min_storage_gb: 50,
                needs_gpu: false,
                needs_database: false,
            }),
            (SystemTier::Standard, TierRequirements {
                min_cpu_cores: 4,
                min_memory_gb: 8,
                min_storage_gb: 100,
                needs_gpu: false,
                needs_database: true,
            }),
            (SystemTier::Advanced, TierRequirements {
                min_cpu_cores: 8,
                min_memory_gb: 16,
                min_storage_gb: 200,
                needs_gpu: true,
                needs_database: true,
            }),
            (SystemTier::Enterprise, TierRequirements {
                min_cpu_cores: 16,
                min_memory_gb: 32,
                min_storage_gb: 500,
                needs_gpu: true,
                needs_database: true,
            }),
        ]);

        Self {
            current_tier: Arc::new(RwLock::new(SystemTier::Minimal)),
            resources: Arc::new(RwLock::new(SystemResources {
                cpu_cores: 0,
                memory_gb: 0,
                storage_gb: 0,
                has_gpu: false,
                has_database: false,
            })),
            requirements,
        }
    }

    /// Initialize system with tiered functionality
    pub async fn initialize(&self) -> Result<SystemTier, anyhow::Error> {
        // Check system resources
        self.check_system_resources().await?;
        
        // Determine highest possible tier
        let tier = self.determine_tier().await?;
        
        // Initialize features for tier
        self.initialize_tier_features(&tier).await?;
        
        // Update current tier
        *self.current_tier.write().await = tier.clone();
        
        info!("System initialized at tier: {:?}", tier);
        Ok(tier)
    }

    /// Check available system resources
    async fn check_system_resources(&self) -> Result<(), anyhow::Error> {
        let mut sys = System::new_all();
        sys.refresh_all();

        let mut resources = self.resources.write().await;
        
        // CPU cores
        resources.cpu_cores = sys.cpus().len() as u32;
        
        // Memory (GB)
        resources.memory_gb = sys.total_memory() / 1024 / 1024 / 1024;
        
        // Storage (GB)
        if let Some(disk) = sys.disks().first() {
            resources.storage_gb = disk.total_space() / 1024 / 1024 / 1024;
        }
        
        // GPU check
        #[cfg(feature = "ml")]
        {
            resources.has_gpu = self.check_gpu_availability().await?;
        }
        
        // Database check
        resources.has_database = self.check_database_availability().await?;

        info!("System resources detected: {:?}", *resources);
        Ok(())
    }

    /// Determine highest possible tier based on resources
    async fn determine_tier(&self) -> Result<SystemTier, anyhow::Error> {
        let resources = self.resources.read().await;
        
        // Check requirements from highest to lowest
        for tier in [SystemTier::Enterprise, SystemTier::Advanced, 
                    SystemTier::Standard, SystemTier::Minimal].iter() {
            if let Some(req) = self.requirements.get(tier) {
                if self.meets_requirements(&resources, req) {
                    return Ok(tier.clone());
                }
            }
        }
        
        Ok(SystemTier::Minimal)
    }

    /// Check if resources meet requirements
    fn meets_requirements(&self, resources: &SystemResources, 
                        requirements: &TierRequirements) -> bool {
        resources.cpu_cores >= requirements.min_cpu_cores &&
        resources.memory_gb >= requirements.min_memory_gb &&
        resources.storage_gb >= requirements.min_storage_gb &&
        (!requirements.needs_gpu || resources.has_gpu) &&
        (!requirements.needs_database || resources.has_database)
    }

    /// Initialize features for specific tier
    async fn initialize_tier_features(&self, tier: &SystemTier) -> Result<(), anyhow::Error> {
        match tier {
            SystemTier::Minimal => {
                self.init_minimal_features().await?;
            },
            SystemTier::Standard => {
                self.init_minimal_features().await?;
                self.init_standard_features().await?;
            },
            SystemTier::Advanced => {
                self.init_minimal_features().await?;
                self.init_standard_features().await?;
                self.init_advanced_features().await?;
            },
            SystemTier::Enterprise => {
                self.init_minimal_features().await?;
                self.init_standard_features().await?;
                self.init_advanced_features().await?;
                self.init_enterprise_features().await?;
            },
        }
        Ok(())
    }

    async fn init_minimal_features(&self) -> Result<(), anyhow::Error> {
        info!("Initializing minimal features");
        // Initialize core Bitcoin functionality
        Ok(())
    }

    async fn init_standard_features(&self) -> Result<(), anyhow::Error> {
        info!("Initializing standard features");
        // Initialize institutional features
        Ok(())
    }

    async fn init_advanced_features(&self) -> Result<(), anyhow::Error> {
        info!("Initializing advanced features");
        // Initialize ML core features
        Ok(())
    }

    async fn init_enterprise_features(&self) -> Result<(), anyhow::Error> {
        info!("Initializing enterprise features");
        // Initialize full ML and advanced features
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tiered_initialization() {
        let system = TieredSystem::new();
        let tier = system.initialize().await.unwrap();
        
        // Verify system initialized at some tier
        assert!(matches!(tier, 
            SystemTier::Minimal | 
            SystemTier::Standard | 
            SystemTier::Advanced | 
            SystemTier::Enterprise
        ));
    }
} 