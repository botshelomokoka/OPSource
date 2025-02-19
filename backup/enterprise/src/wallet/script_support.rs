use bitcoin::{Script, ScriptBuf, Address, Network, Transaction};
use bitcoin::taproot::{TapLeafHash, TaprootBuilder, TaprootSpendInfo};

pub struct ScriptManager {
    network: Network,
    taproot_builder: TaprootBuilder,
}

impl ScriptManager {
    pub fn new(network: Network) -> Self {
        Self {
            network,
            taproot_builder: TaprootBuilder::new(),
        }
    }

    /// Support for different address types
    pub fn generate_address(&self, address_type: AddressType) -> Result<Address, ScriptError> {
        match address_type {
            AddressType::Legacy => self.create_p2pkh_address(),
            AddressType::SegWit => self.create_p2wpkh_address(),
            AddressType::NestedSegWit => self.create_p2sh_p2wpkh_address(),
            AddressType::Taproot => self.create_taproot_address(),
        }
    }

    /// Taproot-specific functionality
    pub fn create_taproot_spend_info(
        &self,
        internal_key: XOnlyPublicKey,
        script_tree: Option<TaprootScriptTree>,
    ) -> Result<TaprootSpendInfo, ScriptError> {
        let builder = TaprootBuilder::new();
        
        if let Some(tree) = script_tree {
            for (script, weight) in tree.scripts {
                builder.add_leaf(weight, script)?;
            }
        }

        builder.finalize(&internal_key)
            .map_err(ScriptError::TaprootError)
    }
} 