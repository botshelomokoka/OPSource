//! # Anya Enterprise Documentation
//! 
//! Enterprise-grade Bitcoin wallet and institutional custody solution with
//! comprehensive support for Layer 2 protocols, Web5, and ML capabilities.

/// Core Documentation Structure
pub struct Documentation {
    core_docs: CoreDocumentation,
    enterprise_docs: EnterpriseDocumentation,
    api_docs: APIDocumentation,
    integration_docs: IntegrationDocumentation,
}

impl Documentation {
    /// Generate comprehensive documentation
    pub fn generate() -> Self {
        Self {
            core_docs: CoreDocumentation::new(),
            enterprise_docs: EnterpriseDocumentation::new(),
            api_docs: APIDocumentation::new(),
            integration_docs: IntegrationDocumentation::new(),
        }
    }
}

/// Core System Documentation
pub struct CoreDocumentation {
    /// Bitcoin Core Integration
    bitcoin_core: BitcoinDocs,
    
    /// Lightning Network Support
    lightning: LightningDocs,
    
    /// Liquid Network Integration
    liquid: LiquidDocs,
    
    /// RGB Protocol Support
    rgb: RGBDocs,
    
    /// DLC Implementation
    dlc: DLCDocs,
    
    /// Taro Protocol Support
    taro: TaroDocs,
}

/// Enterprise Features Documentation
pub struct EnterpriseDocumentation {
    /// Institutional Features
    institutional: InstitutionalDocs,
    
    /// Security Features
    security: SecurityDocs,
    
    /// ML Capabilities
    ml: MLDocs,
    
    /// Web5 Integration
    web5: Web5Docs,
    
    /// Compliance Features
    compliance: ComplianceDocs,
} 