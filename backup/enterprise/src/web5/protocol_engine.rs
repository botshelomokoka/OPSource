pub struct ProtocolEngine {
    protocol_validator: Arc<ProtocolValidator>,
    state_manager: Arc<StateManager>,
    interaction_handler: Arc<InteractionHandler>,
}

impl ProtocolEngine {
    pub async fn process_protocol(
        &self,
        protocol: Web5Protocol,
        context: &ProtocolContext,
    ) -> Result<ProtocolResult, ProtocolError> {
        // 1. Validate Protocol
        self.validate_protocol(&protocol).await?;
        
        // 2. Process Protocol Steps
        let steps = self.process_protocol_steps(
            &protocol,
            context,
        ).await?;
        
        // 3. Handle Interactions
        let interactions = self.handle_protocol_interactions(
            &protocol,
            &steps,
            context,
        ).await?;
        
        // 4. Update Protocol State
        let state_update = self.update_protocol_state(
            &protocol,
            &steps,
            &interactions,
        ).await?;

        Ok(ProtocolResult {
            protocol,
            steps,
            interactions,
            state: state_update,
            timestamp: chrono::Utc::now(),
        })
    }

    async fn process_protocol_steps(
        &self,
        protocol: &Web5Protocol,
        context: &ProtocolContext,
    ) -> Result<Vec<ProtocolStep>, ProtocolError> {
        let mut processed_steps = Vec::new();

        for step in &protocol.steps {
            let processed = match step.step_type {
                StepType::DataExchange => {
                    self.process_data_exchange(step, context).await?
                },
                StepType::IdentityVerification => {
                    self.process_identity_verification(step, context).await?
                },
                StepType::CredentialIssuance => {
                    self.process_credential_issuance(step, context).await?
                },
                StepType::StateTransition => {
                    self.process_state_transition(step, context).await?
                },
            };
            processed_steps.push(processed);
        }

        Ok(processed_steps)
    }
} 