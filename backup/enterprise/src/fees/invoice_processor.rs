pub struct InvoiceProcessor {
    invoice_generator: Arc<InvoiceGenerator>,
    payment_validator: Arc<PaymentValidator>,
    record_keeper: Arc<RecordKeeper>,
}

impl InvoiceProcessor {
    pub async fn create_fee_invoice(
        &self,
        fee: EnterpriseFeeRequest,
    ) -> Result<FeeInvoice, InvoiceError> {
        // 1. Validate Request
        self.validate_invoice_request(&fee).await?;
        
        // 2. Generate Invoice
        let invoice = self.generate_invoice(&fee).await?;
        
        // 3. Setup Payment Methods
        let payment_methods = self.setup_payment_methods(&fee).await?;
        
        // 4. Create Records
        let records = self.create_invoice_records(
            &invoice,
            &payment_methods,
        ).await?;

        Ok(FeeInvoice {
            invoice_id: invoice.id,
            amount: fee.amount,
            payment_methods,
            records,
            status: InvoiceStatus::Created,
            expiry: chrono::Utc::now() + chrono::Duration::hours(24),
        })
    }

    async fn generate_invoice(
        &self,
        fee: &EnterpriseFeeRequest,
    ) -> Result<Invoice, InvoiceError> {
        // Generate Bitcoin invoice
        let btc_invoice = self.invoice_generator
            .generate_bitcoin_invoice(fee)
            .await?;
            
        // Generate Lightning invoice
        let ln_invoice = self.invoice_generator
            .generate_lightning_invoice(fee)
            .await?;

        Ok(Invoice {
            bitcoin: btc_invoice,
            lightning: ln_invoice,
            amount: fee.amount,
            description: fee.description.clone(),
            created_at: chrono::Utc::now(),
        })
    }
} 