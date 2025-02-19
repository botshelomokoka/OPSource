use bitcoin::psbt::{Psbt, PsbtSighashType};
use bitcoin::taproot::{TapLeafHash, TaprootSignature};

pub struct TransactionManager {
    script_manager: ScriptManager,
    signing_service: SigningService,
}

impl TransactionManager {
    /// Handle different signature types including Taproot
    pub async fn sign_transaction(
        &self,
        psbt: &mut Psbt,
        sighash_type: PsbtSighashType,
    ) -> Result<(), TransactionError> {
        for input in psbt.inputs_mut() {
            match input.witness_utxo {
                Some(ref utxo) if utxo.script_pubkey.is_p2tr() => {
                    self.sign_taproot_input(input, sighash_type).await?;
                }
                Some(ref utxo) if utxo.script_pubkey.is_witness_program() => {
                    self.sign_segwit_input(input, sighash_type).await?;
                }
                _ => self.sign_legacy_input(input, sighash_type).await?,
            }
        }
        Ok(())
    }

    /// Taproot-specific signing
    async fn sign_taproot_input(
        &self,
        input: &mut psbt::Input,
        sighash_type: PsbtSighashType,
    ) -> Result<(), TransactionError> {
        let spend_info = self.script_manager.get_taproot_spend_info(&input)?;
        
        // Handle both key path and script path spending
        if let Some(control_block) = &input.tap_script_sig {
            // Script path spending
            self.signing_service.sign_tap_script(
                input,
                spend_info,
                control_block,
                sighash_type,
            ).await?;
        } else {
            // Key path spending
            self.signing_service.sign_tap_key_path(
                input,
                spend_info.internal_key(),
                sighash_type,
            ).await?;
        }
        Ok(())
    }
} 