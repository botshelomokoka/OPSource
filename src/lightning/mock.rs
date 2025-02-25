// Mock implementation of the Lightning Network interface
// Used for testing and development when LDK is not available

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::lightning::interface::{
    LightningInterface, LightningError, LightningResult,
    NodeInfo, ChannelInfo, Invoice, PaymentInfo, PaymentStatus,
    LightningImplementationType
};

use crate::lightning::channel_manager::ChannelManagerWrapper;
use crate::lightning::peer_manager::PeerManagerWrapper;
use crate::lightning::key_manager::KeyManagerWrapper;
use crate::lightning::invoice_manager::InvoiceManager;
use crate::lightning::payment_router::PaymentRouter;
use crate::lightning::payment_executor::PaymentExecutor;

/// Mock implementation of Lightning Network interface
pub struct MockLightningImplementation {
    /// Configuration
    config: Arc<crate::config::Config>,
    
    /// Key manager
    key_manager: KeyManagerWrapper,
    
    /// Channel manager
    channel_manager: Arc<ChannelManagerWrapper>,
    
    /// Peer manager
    peer_manager: Arc<PeerManagerWrapper>,
    
    /// Invoice manager
    invoice_manager: Arc<InvoiceManager>,
    
    /// Payment router
    payment_router: Arc<PaymentRouter>,
    
    /// Payment executor
    payment_executor: Arc<PaymentExecutor>,
    
    /// Reference to Bitcoin interface
    bitcoin_interface: Arc<dyn crate::bitcoin::BitcoinInterface>,
    
    /// Initialization status
    initialized: Mutex<bool>,
}

impl MockLightningImplementation {
    /// Create a new mock Lightning implementation
    pub fn new(config: &crate::config::Config, bitcoin_interface: Arc<dyn crate::bitcoin::BitcoinInterface>) -> Self {
        // Create required components
        let mut key_manager = KeyManagerWrapper::new(config);
        let peer_manager = Arc::new(PeerManagerWrapper::new(config));
        let channel_manager = Arc::new(ChannelManagerWrapper::new(config, bitcoin_interface.clone()));
        let payment_router = Arc::new(PaymentRouter::new(config));
        
        // Initialize key manager
        let _ = key_manager.initialize();
        
        // Create invoice manager with key manager
        let key_manager_arc = Arc::new(key_manager.clone());
        let invoice_manager = Arc::new(InvoiceManager::new(config, key_manager_arc.clone()));
        
        // Create payment executor with all components
        let payment_executor = Arc::new(PaymentExecutor::new(
            config,
            payment_router.clone(),
            invoice_manager.clone(),
            channel_manager.clone(),
            peer_manager.clone()
        ));
        
        MockLightningImplementation {
            config: Arc::new(config.clone()),
            key_manager,
            channel_manager,
            peer_manager,
            invoice_manager,
            payment_router,
            payment_executor,
            bitcoin_interface,
            initialized: Mutex::new(false),
        }
    }
    
    /// Initialize all components
    fn ensure_initialized(&self) -> LightningResult<()> {
        let mut initialized = self.initialized.lock().unwrap();
        
        if !*initialized {
            println!("Initializing Mock Lightning implementation...");
            
            // Initialize components
            let mut peer_manager = self.peer_manager.clone();
            peer_manager.initialize()?;
            
            let mut channel_manager = self.channel_manager.clone();
            channel_manager.initialize()?;
            
            *initialized = true;
            println!("Mock Lightning implementation initialized");
        }
        
        Ok(())
    }
}

impl LightningInterface for MockLightningImplementation {
    fn get_node_info(&self) -> LightningResult<NodeInfo> {
        // Ensure we're initialized
        self.ensure_initialized()?;
        
        // Get node info from key manager
        self.key_manager.get_node_info()
    }
    
    fn connect_peer(&self, node_pubkey: &str, host: &str, port: u16) -> LightningResult<()> {
        // Ensure we're initialized
        self.ensure_initialized()?;
        
        // Connect using peer manager
        self.peer_manager.connect_peer(node_pubkey, host, port)
    }
    
    fn list_peers(&self) -> LightningResult<Vec<NodeInfo>> {
        // Ensure we're initialized
        self.ensure_initialized()?;
        
        // List peers from peer manager
        self.peer_manager.list_peers()
    }
    
    fn open_channel(
        &self,
        node_pubkey: &str,
        capacity: u64,
        push_msat: Option<u64>,
        is_private: bool,
    ) -> LightningResult<ChannelInfo> {
        // Ensure we're initialized
        self.ensure_initialized()?;
        
        // Check if we're connected to the peer
        if !self.peer_manager.is_connected(node_pubkey) {
            return Err(LightningError::ChannelError(
                format!("Not connected to peer {}", node_pubkey)
            ));
        }
        
        // Open channel using channel manager
        self.channel_manager.open_channel(node_pubkey, capacity, push_msat, is_private)
    }
    
    fn list_channels(&self) -> LightningResult<Vec<ChannelInfo>> {
        // Ensure we're initialized
        self.ensure_initialized()?;
        
        // List channels from channel manager
        self.channel_manager.list_channels()
    }
    
    fn close_channel(&self, channel_id: &str, force: bool) -> LightningResult<String> {
        // Ensure we're initialized
        self.ensure_initialized()?;
        
        // Close channel using channel manager
        self.channel_manager.close_channel(channel_id, force)
    }
    
    fn create_invoice(
        &self,
        amount_msat: Option<u64>,
        description: &str,
        expiry: Option<u32>,
    ) -> LightningResult<Invoice> {
        // Ensure we're initialized
        self.ensure_initialized()?;
        
        // Create invoice using invoice manager
        self.invoice_manager.create_invoice(amount_msat, description, expiry)
    }
    
    fn pay_invoice(&self, bolt11: &str, amount_msat: Option<u64>) -> LightningResult<PaymentInfo> {
        // Ensure we're initialized
        self.ensure_initialized()?;
        
        // Pay invoice using payment executor
        self.payment_executor.pay_invoice(bolt11, amount_msat)
    }
    
    fn decode_invoice(&self, bolt11: &str) -> LightningResult<Invoice> {
        // Decode invoice using invoice manager
        self.invoice_manager.decode_invoice(bolt11)
    }
    
    fn get_payment(&self, payment_hash: &str) -> LightningResult<Option<PaymentInfo>> {
        // Ensure we're initialized
        self.ensure_initialized()?;
        
        // Get payment from payment executor
        self.payment_executor.get_payment(payment_hash)
    }
    
    fn list_payments(&self) -> LightningResult<Vec<PaymentInfo>> {
        // Ensure we're initialized
        self.ensure_initialized()?;
        
        // List payments from payment executor
        self.payment_executor.list_payments()
    }
    
    fn implementation_type(&self) -> LightningImplementationType {
        LightningImplementationType::Mock
    }
} 