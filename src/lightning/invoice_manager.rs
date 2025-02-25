// Lightning Network Invoice Manager
// Handles invoice creation, parsing, and storage

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

use crate::lightning::interface::{
    LightningError, LightningResult, Invoice
};

#[cfg(feature = "ldk")]
use lightning_invoice::{
    Invoice as LdkInvoice, 
    InvoiceBuilder, 
    Currency, 
    Description, 
    Bolt11InvoiceDescription,
    SignOrCreationError,
    utils::create_invoice_for_network, 
};

#[cfg(feature = "ldk")]
use bitcoin::secp256k1::{
    Secp256k1, SecretKey
};

use crate::lightning::key_manager::KeyManagerWrapper;

/// Invoice Manager component for handling Lightning invoices
pub struct InvoiceManager {
    /// Stored invoices
    invoices: Mutex<HashMap<String, InvoiceWithStatus>>,
    
    /// Key manager for signing invoices
    key_manager: Arc<KeyManagerWrapper>,
    
    /// Configuration
    config: Arc<crate::config::Config>,
}

/// Invoice with additional status information
#[derive(Clone, Debug)]
pub struct InvoiceWithStatus {
    /// The invoice itself
    pub invoice: Invoice,
    
    /// Whether the invoice has been paid
    pub is_paid: bool,
    
    /// When the invoice was paid (if applicable)
    pub paid_at: Option<u64>,
    
    /// The preimage that was revealed (if paid)
    pub payment_preimage: Option<String>,
}

impl InvoiceManager {
    /// Create a new Invoice Manager
    pub fn new(config: &crate::config::Config, key_manager: Arc<KeyManagerWrapper>) -> Self {
        InvoiceManager {
            invoices: Mutex::new(HashMap::new()),
            key_manager,
            config: Arc::new(config.clone()),
        }
    }
    
    /// Create a new invoice
    pub fn create_invoice(
        &self,
        amount_msat: Option<u64>,
        description: &str,
        expiry: Option<u32>,
    ) -> LightningResult<Invoice> {
        let current_time = self.get_timestamp();
        let expiry_time = expiry.unwrap_or(3600); // Default 1 hour expiry
        
        #[cfg(feature = "ldk")]
        {
            // This would be implemented with actual LDK invoice creation logic
            // using the key_manager to sign the invoice
            
            // For now, create a mock invoice
            let payment_hash = generate_random_bytes_hex(32);
            let bolt11 = self.generate_mock_bolt11(amount_msat, &payment_hash, description);
            
            let invoice = Invoice {
                bolt11,
                payment_hash,
                description: description.to_string(),
                amount_msat,
                expiry: expiry_time,
                timestamp: current_time,
                min_final_cltv_expiry: 40, // Standard value
            };
            
            // Store the invoice
            let mut invoices = self.invoices.lock().unwrap();
            invoices.insert(invoice.payment_hash.clone(), InvoiceWithStatus {
                invoice: invoice.clone(),
                is_paid: false,
                paid_at: None,
                payment_preimage: None,
            });
            
            Ok(invoice)
        }
        
        #[cfg(not(feature = "ldk"))]
        {
            // Create a mock invoice
            let payment_hash = generate_random_bytes_hex(32);
            let bolt11 = self.generate_mock_bolt11(amount_msat, &payment_hash, description);
            
            let invoice = Invoice {
                bolt11,
                payment_hash,
                description: description.to_string(),
                amount_msat,
                expiry: expiry_time,
                timestamp: current_time,
                min_final_cltv_expiry: 40, // Standard value
            };
            
            // Store the invoice
            let mut invoices = self.invoices.lock().unwrap();
            invoices.insert(invoice.payment_hash.clone(), InvoiceWithStatus {
                invoice: invoice.clone(),
                is_paid: false,
                paid_at: None,
                payment_preimage: None,
            });
            
            Ok(invoice)
        }
    }
    
    /// Parse/decode a BOLT11 invoice
    pub fn decode_invoice(&self, bolt11: &str) -> LightningResult<Invoice> {
        #[cfg(feature = "ldk")]
        {
            // In a real implementation, we would use LDK to parse the invoice
            // For now, create a fake decoded invoice
            
            // Try to extract some basic info from the BOLT11 string
            let has_amount = bolt11.contains('m');
            let amount_msat = if has_amount {
                // Simple mock extraction
                if let Some(start) = bolt11.find("lnbc") {
                    let amount_str = bolt11[start+4..].chars()
                        .take_while(|c| c.is_digit(10) || *c == 'm')
                        .collect::<String>();
                    
                    if amount_str.ends_with('m') {
                        let amount = amount_str[..amount_str.len()-1].parse::<u64>().unwrap_or(0);
                        Some(amount)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };
            
            let payment_hash = generate_random_bytes_hex(32);
            
            Ok(Invoice {
                bolt11: bolt11.to_string(),
                payment_hash,
                description: "Decoded invoice".to_string(),
                amount_msat,
                expiry: 3600,
                timestamp: self.get_timestamp(),
                min_final_cltv_expiry: 40,
            })
        }
        
        #[cfg(not(feature = "ldk"))]
        {
            // Mock implementation - create a synthetic invoice
            let has_amount = bolt11.contains('m');
            let amount_msat = if has_amount {
                // Simple mock extraction
                if let Some(start) = bolt11.find("lnbc") {
                    let amount_str = bolt11[start+4..].chars()
                        .take_while(|c| c.is_digit(10) || *c == 'm')
                        .collect::<String>();
                    
                    if amount_str.ends_with('m') {
                        let amount = amount_str[..amount_str.len()-1].parse::<u64>().unwrap_or(0);
                        Some(amount)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };
            
            let payment_hash = generate_random_bytes_hex(32);
            
            Ok(Invoice {
                bolt11: bolt11.to_string(),
                payment_hash,
                description: "Decoded invoice".to_string(),
                amount_msat,
                expiry: 3600,
                timestamp: self.get_timestamp(),
                min_final_cltv_expiry: 40,
            })
        }
    }
    
    /// Check if an invoice exists
    pub fn has_invoice(&self, payment_hash: &str) -> bool {
        let invoices = self.invoices.lock().unwrap();
        invoices.contains_key(payment_hash)
    }
    
    /// Get an invoice by payment hash
    pub fn get_invoice(&self, payment_hash: &str) -> LightningResult<Option<Invoice>> {
        let invoices = self.invoices.lock().unwrap();
        Ok(invoices.get(payment_hash).map(|i| i.invoice.clone()))
    }
    
    /// Get all invoices
    pub fn list_invoices(&self) -> LightningResult<Vec<Invoice>> {
        let invoices = self.invoices.lock().unwrap();
        Ok(invoices.values().map(|i| i.invoice.clone()).collect())
    }
    
    /// Mark an invoice as paid
    pub fn mark_invoice_paid(
        &self, 
        payment_hash: &str, 
        payment_preimage: &str
    ) -> LightningResult<()> {
        let mut invoices = self.invoices.lock().unwrap();
        
        match invoices.get_mut(payment_hash) {
            Some(invoice_status) => {
                invoice_status.is_paid = true;
                invoice_status.paid_at = Some(self.get_timestamp());
                invoice_status.payment_preimage = Some(payment_preimage.to_string());
                Ok(())
            },
            None => Err(LightningError::InvoiceError(
                format!("Invoice not found: {}", payment_hash)
            )),
        }
    }
    
    /// Check if an invoice is paid
    pub fn is_invoice_paid(&self, payment_hash: &str) -> LightningResult<bool> {
        let invoices = self.invoices.lock().unwrap();
        
        match invoices.get(payment_hash) {
            Some(invoice_status) => Ok(invoice_status.is_paid),
            None => Err(LightningError::InvoiceError(
                format!("Invoice not found: {}", payment_hash)
            )),
        }
    }
    
    /// Check if an invoice is expired
    pub fn is_invoice_expired(&self, payment_hash: &str) -> LightningResult<bool> {
        let invoices = self.invoices.lock().unwrap();
        
        match invoices.get(payment_hash) {
            Some(invoice_status) => {
                let invoice = &invoice_status.invoice;
                let now = self.get_timestamp();
                let expiry_time = invoice.timestamp + invoice.expiry as u64;
                Ok(now > expiry_time)
            },
            None => Err(LightningError::InvoiceError(
                format!("Invoice not found: {}", payment_hash)
            )),
        }
    }
    
    /// Get current timestamp
    fn get_timestamp(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
    
    /// Generate a mock BOLT11 invoice string (for testing without LDK)
    fn generate_mock_bolt11(
        &self, 
        amount_msat: Option<u64>, 
        payment_hash: &str,
        description: &str
    ) -> String {
        let network = self.config.bitcoin_network.as_deref().unwrap_or("testnet");
        let prefix = match network {
            "mainnet" | "bitcoin" => "lnbc",
            "testnet" => "lntb",
            "regtest" => "lnbcrt",
            _ => "lntb",
        };
        
        // Create a simplified mock BOLT11 string
        let amount_part = match amount_msat {
            Some(amt) => format!("{}", amt / 1000), // Convert to sat
            None => "any".to_string(),
        };
        
        // Take first 6 chars of description for hrp
        let desc_part = description.chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .take(6)
            .collect::<String>()
            .to_lowercase();
        
        // Create timestamp part - use last 6 digits
        let timestamp = self.get_timestamp() % 1_000_000;
        
        // Create a fakr signature part
        let sig_part = &payment_hash[0..12];
        
        format!("{}{}{}{}{}", prefix, amount_part, desc_part, timestamp, sig_part)
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