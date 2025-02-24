import tf_encrypted as tfe
from anya_ml import FederatedModel
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.asymmetric import padding

def verify_model_update(update, signature, public_key):
    hasher = hashes.Hash(hashes.SHA256())
    hasher.update(update.tobytes())
    digest = hasher.finalize()
    
    public_key.verify(
        signature,
        digest,
        padding.PSS(
            mgf=padding.MGF1(hashes.SHA256()),
            salt_length=padding.PSS.MAX_LENGTH
        ),
        hashes.SHA256()
    )

class PrivacyModel(FederatedModel):
    def __init__(self):
        self.global_model = tfe.keras.Sequential([
            tfe.keras.layers.Dense(64, activation='relu'),
            tfe.keras.layers.Dense(1, activation='sigmoid')
        ])
        
    def aggregate_weights(self, client_weights, signatures, pubkeys):
        for w, sig, pk in zip(client_weights, signatures, pubkeys):
            verify_model_update(w, sig, pk)
        secure_sum = tfe.add_n(client_weights)
        return [w / len(client_weights) for w in secure_sum]

    def train_round(self, data):
        with tfe.protocol.SecureNN():
            return self.global_model.fit(
                data.features, 
                data.labels,
                epochs=5,
                batch_size=32
            )

class FederatedTrainer:
    def __init__(self, model):
        self.model = model
        self.clients = []
        
    def add_client(self, client: Client):
        self.clients.append(client)
    
    def federated_round(self):
        avg_weights = []
        for client in self.clients:
            weights = tfe.reveal(client.get_encrypted_weights())
            avg_weights.append(weights)
        
        global_weights = np.mean(avg_weights, axis=0)
        self.model.set_weights(global_weights)
        
        for client in self.clients:
            client.update_model(global_weights) 