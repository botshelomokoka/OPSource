use crate::security::SecurityManager;
use crate::monitoring::LiquidMonitor;
use anya_core::liquid::LiquidProtocol;

pub struct LiquidServices {
    liquid_protocol: Arc<LiquidProtocol>,
    security_manager: Arc<SecurityManager>,
    liquid_monitor: Arc<LiquidMonitor>,
    service_manager: Arc<ServiceManager>,
}

impl LiquidServices {
    pub async fn process_liquid_service(
        &self,
        request: LiquidServiceRequest,
        context: &ServiceContext,
    ) -> Result<LiquidServiceResult, ServiceError> {
        // 1. Security Validation
        self.security_manager
            .validate_service_request(&request, context)
            .await?;
        
        // 2. Process Service
        let result = match request.service_type {
            ServiceType::AssetManagement(config) => {
                self.handle_asset_management(config, context).await?
            },
            ServiceType::ConfidentialServices(service) => {
                self.handle_confidential_services(service, context).await?
            },
            ServiceType::PegServices(peg) => {
                self.handle_peg_services(peg, context).await?
            },
            ServiceType::IssuanceServices(issue) => {
                self.handle_issuance_services(issue, context).await?
            },
        };
        
        // 3. Monitor Service
        self.liquid_monitor
            .track_service_operation(&result)
            .await?;

        Ok(result)
    }

    async fn handle_confidential_services(
        &self,
        service: ConfidentialService,
        context: &ServiceContext,
    ) -> Result<LiquidServiceResult, ServiceError> {
        // Process confidential service request
        let protocol_request = self.prepare_protocol_request(&service).await?;
        
        // Execute protocol operation
        let protocol_result = self.liquid_protocol
            .process_liquid_protocol(protocol_request, &context.into())
            .await?;
            
        // Process service result
        let service_result = self.process_protocol_result(
            protocol_result,
            &service,
        ).await?;

        Ok(LiquidServiceResult::Confidential(service_result))
    }
} 