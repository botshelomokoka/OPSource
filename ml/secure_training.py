# Gradient signing with TEE
import tf_encrypted as tfe
from tf_encrypted.secure import RemoteConfig
from tf_encrypted.protocol.pond import TFEInputter
from typing import List, Tuple
import numpy as np

class SecureTrainer:
    def __init__(self, tee_endpoint: str, private_key: bytes, public_key: bytes):
        self.tee_config = RemoteConfig(tee_endpoint)
        self.private_key = private_key
        self.public_key = public_key
        tfe.set_config(self.tee_config)
        tfe.set_protocol(tfe.protocol.SecureNN())

    def federated_averaging(self, client_updates: List[np.ndarray]) -> np.ndarray:
        signed_updates = [self._sign_update(update) for update in client_updates]
        return self._secure_aggregate(signed_updates)
    
    def _sign_update(self, update: np.ndarray) -> Tuple[np.ndarray, bytes]:
        with tfe.protocol.Pond():
            sig = tfe.sign(update, self.private_key)
            return (update, sig.numpy())
    
    def _secure_aggregate(self, signed_updates: List[Tuple[np.ndarray, bytes]]) -> np.ndarray:
        validated_updates = []
        for update, sig in signed_updates:
            if self._verify_update(update, sig):
                validated_updates.append(update)
        return np.mean(validated_updates, axis=0)
    
    def _verify_update(self, update: np.ndarray, signature: bytes) -> bool:
        with tfe.protocol.Pond():
            verified = tfe.verify(update, signature, self.public_key)
            return verified.numpy()

# Add missing imports and initialize TFE
from tf_encrypted.keras import initializers