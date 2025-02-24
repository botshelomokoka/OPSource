# Gradient signing with TEE
from tf_encrypted.secure import RemoteConfig

class SecureTrainer:
    def __init__(self, tee_endpoint):
        self.tee_config = RemoteConfig(tee_endpoint)
        
    def federated_averaging(self, client_updates):
        with tfe.protocol.Pond(*self.tee_config):
            signed_updates = [
                self._sign_update(update) 
                for update in client_updates
            ]
            return self._secure_aggregate(signed_updates)
    
    def _sign_update(self, update):
        with self.tee_config:
            sig = tfe.sign(update, self.private_key)
            return (update, sig)
    
    def _verify_update(self, update, sig, public_key):
        return tfe.verify(update, sig, public_key) 