use std::time::Duration;
use tokio::time::sleep;
use governor::{Quota, RateLimiter};

pub struct SecurityMiddleware {
    rate_limiter: RateLimiter,
    transaction_validator: TransactionValidator,
}

impl SecurityMiddleware {
    pub async fn validate_request(&self, request: &Request) -> Result<(), SecurityError> {
        // Rate limiting
        if !self.rate_limiter.check() {
            return Err(SecurityError::RateLimitExceeded);
        }
        
        // Input validation
        self.validate_input(request)?;
        
        // Authentication check
        self.verify_authentication(request)?;
        
        Ok(())
    }
    
    pub async fn validate_transaction(&self, tx: &Transaction) -> Result<(), SecurityError> {
        // Comprehensive transaction validation
        self.transaction_validator.validate_inputs(tx)?;
        self.transaction_validator.validate_outputs(tx)?;
        self.transaction_validator.check_fee_rate(tx)?;
        
        Ok(())
    }
} 