// Lightning Network module
// Provides a unified interface for Lightning Network operations

pub mod interface;
pub mod mock;
pub mod ldk;
pub mod channel_manager;
pub mod peer_manager;
pub mod key_manager;
pub mod invoice_manager;
pub mod payment_router;
pub mod payment_executor;
pub mod bitcoin_bridge;

use std::sync::Arc;
use crate::config::Config;
use interface::{
    LightningInterface,
    LightningImplementationType,
    NodeInfo, ChannelInfo, Invoice, PaymentInfo,
};

/// Create a Lightning Network interface based on the configuration
pub fn create_lightning_interface(
    config: &Config,
    bitcoin_interface: Arc<dyn crate::bitcoin::BitcoinInterface>,
) -> Arc<dyn LightningInterface> {
    match config.get_lightning_implementation_type() {
        LightningImplementationType::LDK => {
            #[cfg(feature = "ldk")]
            {
                Arc::new(ldk::LdkLightningImplementation::new(config, bitcoin_interface))
            }
            #[cfg(not(feature = "ldk"))]
            {
                println!("Warning: LDK implementation requested but feature not enabled, using mock implementation");
                Arc::new(mock::MockLightningImplementation::new(config, bitcoin_interface))
            }
        }
        LightningImplementationType::Mock => {
            Arc::new(mock::MockLightningImplementation::new(config, bitcoin_interface))
        }
    }
}

/// Initialize the Lightning module
pub fn init() {
    println!("Initializing Lightning Network module");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::bitcoin;

    #[test]
    fn test_create_lightning_interface() {
        let config = Config::default();
        let bitcoin_interface = bitcoin::get_current_bitcoin_interface(&config);
        
        let lightning = create_lightning_interface(&config, bitcoin_interface);
        assert_eq!(lightning.implementation_type(), LightningImplementationType::Mock);
        
        let node_info = lightning.get_node_info().unwrap();
        assert!(!node_info.pubkey.is_empty());
    }
    
    #[test]
    fn test_channel_manager() {
        use super::channel_manager::ChannelManagerWrapper;
        
        let config = Config::default();
        let bitcoin_interface = bitcoin::get_current_bitcoin_interface(&config);
        
        let mut channel_manager = ChannelManagerWrapper::new(&config, bitcoin_interface);
        
        #[cfg(feature = "ldk")]
        {
            // Not directly testing LDK functionality in this test
        }
        
        #[cfg(not(feature = "ldk"))]
        {
            // Test the mock implementation
            channel_manager.initialize().unwrap();
            
            // List channels
            let channels = channel_manager.list_channels().unwrap();
            assert!(!channels.is_empty());
            
            // Open a new channel
            let peer_pubkey = "02eec7245d6b7d2ccb30380bfbe2a3648cd7a942653f5aa340edcea1f283686619";
            let channel = channel_manager.open_channel(peer_pubkey, 100_000, None, false).unwrap();
            assert_eq!(channel.remote_pubkey, peer_pubkey);
            
            // Close the channel
            let result = channel_manager.close_channel(&channel.channel_id, false).unwrap();
            assert!(!result.is_empty());
        }
    }
    
    #[test]
    fn test_peer_manager() {
        use super::peer_manager::PeerManagerWrapper;
        
        let config = Config::default();
        
        let mut peer_manager = PeerManagerWrapper::new(&config);
        
        #[cfg(not(feature = "ldk"))]
        {
            // Test the mock implementation
            peer_manager.initialize().unwrap();
            
            // List peers
            let peers = peer_manager.list_peers().unwrap();
            assert!(!peers.is_empty());
            
            // Connect to a new peer
            let peer_pubkey = "03f02d965ffe0315fd7470b35a09584edb7ae4d2049c7e78584cc2f476db2c5bed";
            peer_manager.connect_peer(peer_pubkey, "127.0.0.1", 9735).unwrap();
            
            // Check if connected
            assert!(peer_manager.is_connected(peer_pubkey));
            
            // Get peer info
            let peer_info = peer_manager.get_peer_info(peer_pubkey).unwrap();
            assert_eq!(peer_info.pubkey, peer_pubkey);
            
            // Disconnect
            peer_manager.disconnect_peer(peer_pubkey).unwrap();
            assert!(!peer_manager.is_connected(peer_pubkey));
        }
    }
    
    #[test]
    fn test_key_manager() {
        use super::key_manager::KeyManagerWrapper;
        
        let config = Config::default();
        
        let mut key_manager = KeyManagerWrapper::new(&config);
        
        #[cfg(not(feature = "ldk"))]
        {
            // Test the mock implementation
            key_manager.initialize().unwrap();
            
            // Get node info
            let node_info = key_manager.get_node_info().unwrap();
            assert!(!node_info.pubkey.is_empty());
            
            // Update node info
            let mut updated_info = node_info.clone();
            updated_info.alias = Some("Updated Node".to_string());
            key_manager.update_node_info(updated_info).unwrap();
            
            // Verify it was updated but pubkey remains the same
            let new_info = key_manager.get_node_info().unwrap();
            assert_eq!(new_info.alias, Some("Updated Node".to_string()));
            assert_eq!(new_info.pubkey, node_info.pubkey);
        }
    }
    
    #[test]
    fn test_invoice_manager() {
        use super::invoice_manager::InvoiceManager;
        use super::key_manager::KeyManagerWrapper;
        use std::sync::Arc;
        
        let config = Config::default();
        
        let mut key_manager = KeyManagerWrapper::new(&config);
        
        #[cfg(not(feature = "ldk"))]
        {
            // Initialize key manager
            key_manager.initialize().unwrap();
        }
        
        let key_manager_arc = Arc::new(key_manager);
        let invoice_manager = InvoiceManager::new(&config, key_manager_arc);
        
        // Create an invoice
        let invoice = invoice_manager.create_invoice(Some(50_000), "Test payment", None).unwrap();
        
        // Verify invoice fields
        assert_eq!(invoice.description, "Test payment");
        assert_eq!(invoice.amount_msat, Some(50_000));
        assert!(!invoice.bolt11.is_empty());
        assert!(!invoice.payment_hash.is_empty());
        
        // Decode the invoice we just created
        let decoded = invoice_manager.decode_invoice(&invoice.bolt11).unwrap();
        
        // Check invoice exists
        assert!(invoice_manager.has_invoice(&invoice.payment_hash));
        
        // Mark as paid
        let preimage = "0000111122223333444455556666777788889999aaaabbbbccccddddeeeeffff";
        invoice_manager.mark_invoice_paid(&invoice.payment_hash, preimage).unwrap();
        
        // Check if paid
        assert!(invoice_manager.is_invoice_paid(&invoice.payment_hash).unwrap());
    }
    
    #[test]
    fn test_payment_router() {
        use super::payment_router::{PaymentRouter, PaymentRoute};
        
        let config = Config::default();
        
        let router = PaymentRouter::new(&config);
        
        // Use the predefined nodes from the mock data
        let source = "02eadbd9e7557375161df8b646776a547c5097cc8288021e9ee72cb33327f912cd";
        let destination = "035566252e83e2a30ec88140ea7948d505615f057b0e4c186a854cfbef365ea3c5";
        
        // Find a route
        let route = router.find_route(source, destination, 100_000, 144).unwrap();
        
        // Verify the route starts at source and ends at destination
        assert!(!route.hops.is_empty());
        assert_eq!(route.hops.first().unwrap().src_node_id, source);
        assert_eq!(route.hops.last().unwrap().dest_node_id, destination);
        
        // Verify total amount and fee
        assert_eq!(route.total_amount_msat, 100_000);
        assert!(route.total_fee_msat > 0);
    }
    
    #[test]
    fn test_payment_executor() {
        use super::payment_executor::PaymentExecutor;
        use super::payment_router::PaymentRouter;
        use super::invoice_manager::InvoiceManager;
        use super::channel_manager::ChannelManagerWrapper;
        use super::peer_manager::PeerManagerWrapper;
        use super::key_manager::KeyManagerWrapper;
        use std::sync::Arc;
        
        let config = Config::default();
        let bitcoin_interface = bitcoin::get_current_bitcoin_interface(&config);
        
        // Create and initialize components
        let mut key_manager = KeyManagerWrapper::new(&config);
        
        #[cfg(not(feature = "ldk"))]
        {
            key_manager.initialize().unwrap();
        }
        
        let key_manager_arc = Arc::new(key_manager);
        let invoice_manager = Arc::new(InvoiceManager::new(&config, key_manager_arc));
        let router = Arc::new(PaymentRouter::new(&config));
        let channel_manager = Arc::new(ChannelManagerWrapper::new(&config, bitcoin_interface.clone()));
        let peer_manager = Arc::new(PeerManagerWrapper::new(&config));
        
        #[cfg(not(feature = "ldk"))]
        {
            channel_manager.initialize().unwrap();
            peer_manager.initialize().unwrap();
        }
        
        // Create payment executor
        let executor = PaymentExecutor::new(
            &config,
            router,
            invoice_manager.clone(),
            channel_manager,
            peer_manager
        );
        
        // Create an invoice first
        let invoice = invoice_manager.create_invoice(Some(50_000), "Test payment", None).unwrap();
        
        // Pay the invoice
        let payment = executor.pay_invoice(&invoice.bolt11, None).unwrap();
        
        // Verify payment
        assert_eq!(payment.amount_msat, 50_000);
        assert_eq!(payment.status, interface::PaymentStatus::Succeeded);
        
        // Get payment details
        let payment_info = executor.get_payment(&payment.payment_hash).unwrap().unwrap();
        assert_eq!(payment_info.payment_hash, payment.payment_hash);
        
        // List payments
        let payments = executor.list_payments().unwrap();
        assert!(!payments.is_empty());
    }
} 