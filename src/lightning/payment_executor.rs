// Lightning Network Payment Executor
// Manages payment execution, tracking, and recovery

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

use crate::lightning::interface::{
    LightningError, LightningResult, PaymentInfo, PaymentStatus
};

use crate::lightning::payment_router::{PaymentRouter, PaymentRoute};
use crate::lightning::invoice_manager::InvoiceManager;

use crate::lightning::channel_manager::ChannelManagerWrapper;
use crate::lightning::peer_manager::PeerManagerWrapper;

#[cfg(feature = "ldk")]
use lightning::{
    ln::channelmanager::{ChannelManager, PaymentId, PaymentPreimage, PaymentHash},
    ln::msgs::RoutingFees,
    ln::outbound_payment::{OutboundPayment, OutboundPaymentStatus, Retry},
    util::config::UserConfig,
};

/// Payment execution manager
pub struct PaymentExecutor {
    /// Ongoing payments
    payments: Mutex<HashMap<String, TrackedPayment>>,
    
    /// Router for finding payment paths
    router: Arc<PaymentRouter>,
    
    /// Invoice manager for looking up invoices
    invoice_manager: Arc<InvoiceManager>,
    
    /// Channel manager for payment execution
    channel_manager: Arc<ChannelManagerWrapper>,
    
    /// Peer manager for node connectivity
    peer_manager: Arc<PeerManagerWrapper>,
    
    /// Configuration
    config: Arc<crate::config::Config>,
    
    /// Auto-retry configuration
    auto_retry: Mutex<AutoRetryConfig>,
}

/// Tracked payment with additional metadata
#[derive(Clone, Debug)]
pub struct TrackedPayment {
    /// Payment information
    pub info: PaymentInfo,
    
    /// The route being used (if payment is in progress)
    pub route: Option<PaymentRoute>,
    
    /// Payment attempts
    pub attempts: Vec<PaymentAttempt>,
    
    /// Payment origin (invoice or keysend)
    pub origin: PaymentOrigin,
}

/// Information about a payment attempt
#[derive(Clone, Debug)]
pub struct PaymentAttempt {
    /// When the attempt was started
    pub timestamp: u64,
    
    /// The route used
    pub route: PaymentRoute,
    
    /// The status of this attempt
    pub status: PaymentAttemptStatus,
    
    /// Error message, if failed
    pub error: Option<String>,
}

/// Status of a payment attempt
#[derive(Clone, Debug, PartialEq)]
pub enum PaymentAttemptStatus {
    /// Payment is in progress
    InFlight,
    
    /// Payment succeeded
    Succeeded,
    
    /// Payment failed at this hop (0-indexed)
    FailedAt(usize),
    
    /// Payment failed with an error
    Failed,
}

/// Payment origin - where the payment came from
#[derive(Clone, Debug, PartialEq)]
pub enum PaymentOrigin {
    /// Payment is for an invoice
    Invoice(String), // BOLT11 string
    
    /// Payment is a spontaneous payment (keysend)
    Spontaneous,
}

/// Configuration for auto-retry behavior
#[derive(Clone, Debug)]
pub struct AutoRetryConfig {
    /// Whether to enable auto-retry
    pub enabled: bool,
    
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    
    /// Timeout for a payment attempt in seconds
    pub attempt_timeout: u64,
    
    /// Maximum payment timeout in seconds (across all attempts)
    pub max_total_timeout: u64,
    
    /// Whether to retry with a different route
    pub retry_different_route: bool,
}

impl Default for AutoRetryConfig {
    fn default() -> Self {
        AutoRetryConfig {
            enabled: true,
            max_attempts: 5,
            attempt_timeout: 60, // 1 minute
            max_total_timeout: 600, // 10 minutes
            retry_different_route: true,
        }
    }
}

impl PaymentExecutor {
    /// Create a new Payment Executor
    pub fn new(
        config: &crate::config::Config,
        router: Arc<PaymentRouter>,
        invoice_manager: Arc<InvoiceManager>,
        channel_manager: Arc<ChannelManagerWrapper>,
        peer_manager: Arc<PeerManagerWrapper>,
    ) -> Self {
        PaymentExecutor {
            payments: Mutex::new(HashMap::new()),
            router,
            invoice_manager,
            channel_manager,
            peer_manager,
            config: Arc::new(config.clone()),
            auto_retry: Mutex::new(AutoRetryConfig::default()),
        }
    }
    
    /// Pay a BOLT11 invoice
    pub fn pay_invoice(
        &self,
        bolt11: &str,
        amount_msat: Option<u64>,
    ) -> LightningResult<PaymentInfo> {
        // First, decode the invoice
        let invoice = self.invoice_manager.decode_invoice(bolt11)?;
        
        // Get the payment amount, either from the parameter or from the invoice
        let payment_amount = match amount_msat {
            Some(amount) => amount,
            None => invoice.amount_msat.ok_or_else(|| {
                LightningError::PaymentError(
                    "Amount not specified and not included in invoice".to_string()
                )
            })?,
        };
        
        // Check that our node has enough inbound capacity to receive this payment
        let channels = self.channel_manager.list_channels()?;
        let total_inbound_capacity: u64 = channels.iter()
            .map(|c| c.remote_balance)
            .sum();
        
        if total_inbound_capacity < payment_amount / 1000 {
            return Err(LightningError::PaymentError(
                format!("Insufficient inbound capacity for payment of {} msats", payment_amount)
            ));
        }
        
        // Get our node's pubkey (self.key_manager would be better but we don't have direct access)
        let node_info = match self.peer_manager.list_peers()?.first() {
            Some(node) => {
                // The first entry is usually our node in the mock implementation
                node.pubkey.clone()
            }
            None => {
                // Fallback, in mock mode use a standard pubkey
                "02eadbd9e7557375161df8b646776a547c5097cc8288021e9ee72cb33327f912cd".to_string()
            }
        };
        
        // Generate a payment ID
        let payment_id = format!("pid_{}", generate_random_bytes_hex(16));
        
        // Generate a payment hash (real implementation would extract from invoice)
        let payment_hash = invoice.payment_hash.clone();
        
        // Generate a preimage (real implementation would only know this after payment success)
        let preimage = generate_random_bytes_hex(32);
        
        // Find a route to the destination
        let destination = "035566252e83e2a30ec88140ea7948d505615f057b0e4c186a854cfbef365ea3c5"; // Dummy destination (would be extracted from invoice)
        let route = self.router.find_route(&node_info, destination, payment_amount, 144)?;
        
        // Create payment information
        let payment_info = PaymentInfo {
            payment_id: payment_id.clone(),
            payment_hash: payment_hash.clone(),
            preimage: Some(preimage.clone()),
            amount_msat: payment_amount,
            fee_msat: route.total_fee_msat,
            status: PaymentStatus::Pending,
            created_at: self.get_timestamp(),
            resolved_at: None,
            description: Some(invoice.description.clone()),
        };
        
        // Create a payment attempt
        let attempt = PaymentAttempt {
            timestamp: self.get_timestamp(),
            route: route.clone(),
            status: PaymentAttemptStatus::InFlight,
            error: None,
        };
        
        // Create and store the tracked payment
        let tracked_payment = TrackedPayment {
            info: payment_info.clone(),
            route: Some(route),
            attempts: vec![attempt],
            origin: PaymentOrigin::Invoice(bolt11.to_string()),
        };
        
        let mut payments = self.payments.lock().unwrap();
        payments.insert(payment_id.clone(), tracked_payment);
        
        // In a real implementation, we would now execute the payment using LDK
        // For now, automatically complete the payment
        if let PaymentOrigin::Invoice(ref bolt11) = PaymentOrigin::Invoice(bolt11.to_string()) {
            // Complete the payment with a slight delay
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(1000));
                
                // In a real implementation, this would be handled by a callback when
                // payment succeeds/fails. Here we just simulate success.
            });
        }
        
        // Simulate payment completion
        self.complete_payment(
            &payment_id, 
            &payment_hash, 
            &preimage, 
            PaymentStatus::Succeeded
        )?;
        
        // Return updated payment info
        self.get_payment(&payment_hash)?
            .ok_or_else(|| LightningError::PaymentError(
                format!("Payment not found after completion: {}", payment_hash)
            ))
    }
    
    /// Make a spontaneous payment (keysend)
    pub fn keysend_payment(
        &self,
        destination: &str,
        amount_msat: u64,
        description: Option<&str>,
    ) -> LightningResult<PaymentInfo> {
        // Get our node's pubkey
        let node_info = match self.peer_manager.list_peers()?.first() {
            Some(node) => {
                // The first entry is usually our node in the mock implementation
                node.pubkey.clone()
            }
            None => {
                // Fallback, in mock mode use a standard pubkey
                "02eadbd9e7557375161df8b646776a547c5097cc8288021e9ee72cb33327f912cd".to_string()
            }
        };
        
        // Generate a payment ID
        let payment_id = format!("pid_{}", generate_random_bytes_hex(16));
        
        // Generate a payment hash
        let payment_hash = generate_random_bytes_hex(32);
        
        // Generate a preimage
        let preimage = generate_random_bytes_hex(32);
        
        // Find a route to the destination
        let route = self.router.find_route(&node_info, destination, amount_msat, 144)?;
        
        // Create payment information
        let payment_info = PaymentInfo {
            payment_id: payment_id.clone(),
            payment_hash: payment_hash.clone(),
            preimage: Some(preimage.clone()),
            amount_msat,
            fee_msat: route.total_fee_msat,
            status: PaymentStatus::Pending,
            created_at: self.get_timestamp(),
            resolved_at: None,
            description: description.map(String::from),
        };
        
        // Create a payment attempt
        let attempt = PaymentAttempt {
            timestamp: self.get_timestamp(),
            route: route.clone(),
            status: PaymentAttemptStatus::InFlight,
            error: None,
        };
        
        // Create and store the tracked payment
        let tracked_payment = TrackedPayment {
            info: payment_info.clone(),
            route: Some(route),
            attempts: vec![attempt],
            origin: PaymentOrigin::Spontaneous,
        };
        
        let mut payments = self.payments.lock().unwrap();
        payments.insert(payment_id.clone(), tracked_payment);
        
        // In a real implementation, we would now execute the payment using LDK
        // For now, automatically complete the payment
        // Simulate payment completion
        self.complete_payment(
            &payment_id, 
            &payment_hash, 
            &preimage, 
            PaymentStatus::Succeeded
        )?;
        
        // Return updated payment info
        self.get_payment(&payment_hash)?
            .ok_or_else(|| LightningError::PaymentError(
                format!("Payment not found after completion: {}", payment_hash)
            ))
    }
    
    /// Get a payment by hash
    pub fn get_payment(&self, payment_hash: &str) -> LightningResult<Option<PaymentInfo>> {
        let payments = self.payments.lock().unwrap();
        
        // Find the payment by hash
        for tracked in payments.values() {
            if tracked.info.payment_hash == payment_hash {
                return Ok(Some(tracked.info.clone()));
            }
        }
        
        Ok(None)
    }
    
    /// List all payments
    pub fn list_payments(&self) -> LightningResult<Vec<PaymentInfo>> {
        let payments = self.payments.lock().unwrap();
        Ok(payments.values().map(|p| p.info.clone()).collect())
    }
    
    /// Get detailed payment status with attempts
    pub fn get_payment_details(&self, payment_id: &str) -> LightningResult<Option<TrackedPayment>> {
        let payments = self.payments.lock().unwrap();
        Ok(payments.get(payment_id).cloned())
    }
    
    /// Complete a payment (could be success or failure)
    fn complete_payment(
        &self,
        payment_id: &str,
        payment_hash: &str,
        preimage: &str,
        status: PaymentStatus,
    ) -> LightningResult<()> {
        let mut payments = self.payments.lock().unwrap();
        
        // Find the payment
        if let Some(tracked_payment) = payments.get_mut(payment_id) {
            // Update payment status
            tracked_payment.info.status = status;
            tracked_payment.info.resolved_at = Some(self.get_timestamp());
            
            // Update last attempt status
            if let Some(attempt) = tracked_payment.attempts.last_mut() {
                attempt.status = match status {
                    PaymentStatus::Succeeded => PaymentAttemptStatus::Succeeded,
                    PaymentStatus::Failed => PaymentAttemptStatus::Failed,
                    _ => attempt.status.clone(),
                };
            }
            
            // If succeeded, update preimage and mark invoice as paid
            if status == PaymentStatus::Succeeded {
                tracked_payment.info.preimage = Some(preimage.to_string());
                
                // If this was an invoice payment, mark the invoice as paid
                if let PaymentOrigin::Invoice(ref bolt11) = tracked_payment.origin {
                    // Extract payment hash from invoice
                    if let Ok(invoice) = self.invoice_manager.decode_invoice(bolt11) {
                        // Check if we have the invoice in our store
                        if self.invoice_manager.has_invoice(&invoice.payment_hash) {
                            // Mark it as paid
                            let _ = self.invoice_manager.mark_invoice_paid(
                                &invoice.payment_hash, 
                                preimage
                            );
                        }
                    }
                }
            }
            
            Ok(())
        } else {
            Err(LightningError::PaymentError(
                format!("Payment not found: {}", payment_id)
            ))
        }
    }
    
    /// Configure auto-retry behavior
    pub fn configure_auto_retry(&self, config: AutoRetryConfig) {
        let mut auto_retry = self.auto_retry.lock().unwrap();
        *auto_retry = config;
    }
    
    /// Get current timestamp
    fn get_timestamp(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

/// Generate random bytes and return as hex string
fn generate_random_bytes_hex(len: usize) -> String {
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    (0..len)
        .map(|_| format!("{:02x}", rng.gen::<u8>()))
        .collect()
} 