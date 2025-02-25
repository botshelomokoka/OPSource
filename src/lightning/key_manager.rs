// Lightning Network Key Manager
// Handles private key management, node identity, and secure storage

use std::sync::{Arc, Mutex};
use std::path::{Path, PathBuf};
use std::fs;
use std::io;

use crate::lightning::interface::{
    LightningError, LightningResult, NodeInfo
};

#[cfg(feature = "ldk")]
use lightning::{
    chain::keysinterface::{KeysManager, KeysInterface, InMemorySigner},
    util::ser::{Readable, Writeable},
};

/// LDK Key Manager wrapper
pub struct KeyManagerWrapper {
    /// LDK Keys Manager
    #[cfg(feature = "ldk")]
    keys_manager: Mutex<Option<Arc<KeysManager>>>,
    
    /// Node info
    node_info: Mutex<NodeInfo>,
    
    /// Configuration
    config: Arc<crate::config::Config>,
    
    /// Data directory
    data_dir: PathBuf,
}

impl KeyManagerWrapper {
    /// Create a new Key Manager wrapper
    pub fn new(config: &crate::config::Config) -> Self {
        let data_dir = config.lightning_data_dir.clone()
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                let base_dir = config.bitcoin_data_dir.clone()
                    .unwrap_or_else(|| "./.ldk".to_string());
                let mut path = PathBuf::from(base_dir);
                path.push("lightning");
                path
            });
        
        // Create a default node info
        let node_pubkey = config.lightning_node_pubkey.clone()
            .unwrap_or_else(|| {
                // Generate a placeholder pubkey
                "02eadbd9e7557375161df8b646776a547c5097cc8288021e9ee72cb33327f912cd".to_string()
            });
        
        let node_info = NodeInfo {
            pubkey: node_pubkey.clone(),
            addresses: vec![config.lightning_listen_addr.clone()
                .unwrap_or_else(|| "127.0.0.1:9735".to_string())],
            alias: Some("LDK Lightning Node".to_string()),
            color: Some("#3399FF".to_string()),
            features: vec![
                "option_static_remotekey".to_string(),
                "option_anchor_outputs".to_string(),
                "option_route_blinding".to_string(),
            ],
        };
        
        KeyManagerWrapper {
            #[cfg(feature = "ldk")]
            keys_manager: Mutex::new(None),
            node_info: Mutex::new(node_info),
            config: Arc::new(config.clone()),
            data_dir,
        }
    }
    
    /// Initialize the key manager
    #[cfg(feature = "ldk")]
    pub fn initialize(&mut self) -> LightningResult<Arc<KeysManager>> {
        // Create the data directory if it doesn't exist
        if !self.data_dir.exists() {
            fs::create_dir_all(&self.data_dir).map_err(|e| {
                LightningError::ImplementationError(format!("Failed to create data directory: {}", e))
            })?;
        }
        
        // Check if we have an existing seed file
        let seed_path = self.data_dir.join("keys_seed.dat");
        
        let seed = if seed_path.exists() {
            // Load existing seed
            self.load_seed(&seed_path)?
        } else {
            // Generate a new seed
            let mut seed = [0u8; 32];
            get_random_bytes(&mut seed).map_err(|e| {
                LightningError::ImplementationError(format!("Failed to generate random seed: {}", e))
            })?;
            
            // Save the seed
            self.save_seed(&seed_path, &seed)?;
            
            seed
        };
        
        // Create the keys manager with the seed
        let keys_manager = Arc::new(KeysManager::new(
            &seed,
            0, // Starting timestamp for ECDH nonces
            0, // Starting timestamp for transactions
        ));
        
        // Update our node info with the actual pubkey from the keys
        #[cfg(feature = "ldk")]
        {
            let node_id = hex::encode(keys_manager.get_node_id().serialize());
            let mut node_info = self.node_info.lock().unwrap();
            node_info.pubkey = node_id;
        }
        
        // Store the keys manager
        *self.keys_manager.lock().unwrap() = Some(Arc::clone(&keys_manager));
        
        println!("Initialized Lightning key manager with node ID: {}", 
                 self.node_info.lock().unwrap().pubkey);
        
        Ok(keys_manager)
    }
    
    #[cfg(not(feature = "ldk"))]
    pub fn initialize(&mut self) -> LightningResult<()> {
        // Mock implementation - just ensure the data directory exists
        if !self.data_dir.exists() {
            fs::create_dir_all(&self.data_dir).map_err(|e| {
                LightningError::ImplementationError(format!("Failed to create data directory: {}", e))
            })?;
        }
        
        println!("Initialized Lightning key manager (mock) with node ID: {}", 
                 self.node_info.lock().unwrap().pubkey);
        
        Ok(())
    }
    
    /// Get node information
    pub fn get_node_info(&self) -> LightningResult<NodeInfo> {
        let node_info = self.node_info.lock().unwrap();
        Ok(node_info.clone())
    }
    
    /// Update node information
    pub fn update_node_info(&self, node_info: NodeInfo) -> LightningResult<()> {
        let mut current_node_info = self.node_info.lock().unwrap();
        
        // Keep the current pubkey - it's derived from our private key
        let pubkey = current_node_info.pubkey.clone();
        
        // Update everything else
        *current_node_info = node_info;
        current_node_info.pubkey = pubkey;
        
        Ok(())
    }
    
    /// Get the data directory
    pub fn get_data_dir(&self) -> &Path {
        &self.data_dir
    }
    
    // Helper methods for key operations
    
    /// Load a seed from a file
    #[cfg(feature = "ldk")]
    fn load_seed(&self, path: &Path) -> LightningResult<[u8; 32]> {
        let mut seed = [0u8; 32];
        
        let mut file = fs::File::open(path).map_err(|e| {
            LightningError::ImplementationError(format!("Failed to open seed file: {}", e))
        })?;
        
        io::Read::read_exact(&mut file, &mut seed).map_err(|e| {
            LightningError::ImplementationError(format!("Failed to read seed file: {}", e))
        })?;
        
        Ok(seed)
    }
    
    /// Save a seed to a file
    #[cfg(feature = "ldk")]
    fn save_seed(&self, path: &Path, seed: &[u8; 32]) -> LightningResult<()> {
        // Create with restrictive permissions
        let mut file = fs::File::create(path).map_err(|e| {
            LightningError::ImplementationError(format!("Failed to create seed file: {}", e))
        })?;
        
        io::Write::write_all(&mut file, seed).map_err(|e| {
            LightningError::ImplementationError(format!("Failed to write seed file: {}", e))
        })?;
        
        // On Unix systems, set permissions to 600 (read/write for owner only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::metadata(path).map_err(|e| {
                LightningError::ImplementationError(format!("Failed to get file metadata: {}", e))
            })?;
            let mut permissions = metadata.permissions();
            permissions.set_mode(0o600);
            fs::set_permissions(path, permissions).map_err(|e| {
                LightningError::ImplementationError(format!("Failed to set file permissions: {}", e))
            })?;
        }
        
        Ok(())
    }
}

/// Get random bytes for seed generation
fn get_random_bytes(dest: &mut [u8]) -> Result<(), Box<dyn std::error::Error>> {
    use rand::{thread_rng, RngCore};
    thread_rng().fill_bytes(dest);
    Ok(())
} 