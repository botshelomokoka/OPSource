"""
Test suite for OPSource API
"""
import pytest
import requests
from fastapi.testclient import TestClient
from main import app

client = TestClient(app)

def test_root_endpoint():
    """Test root endpoint"""
    response = client.get("/")
    assert response.status_code == 200
    data = response.json()
    assert "name" in data
    assert "version" in data
    assert "status" in data
    assert "bitcoin_network" in data

def test_health_check():
    """Test health check endpoint"""
    response = client.get("/health")
    assert response.status_code == 200
    data = response.json()
    assert data["status"] == "healthy"
    
    # Test Bitcoin components
    assert "bitcoin" in data["components"]
    bitcoin = data["components"]["bitcoin"]
    assert "network" in bitcoin
    assert "pruned" in bitcoin
    assert "taproot" in bitcoin
    assert "dlc" in bitcoin

    # Test Web5 components
    assert "web5" in data["components"]
    web5 = data["components"]["web5"]
    assert "did_enabled" in web5
    assert "handshake_enabled" in web5
    assert "decentralized_storage" in web5

    # Test Layer2 components
    assert "layer2" in data["components"]
    layer2 = data["components"]["layer2"]
    assert "lightning" in layer2
    assert "rgb" in layer2
    assert "rsk" in layer2
    assert "stacks" in layer2

    # Test AI components
    assert "ai" in data["components"]
    ai = data["components"]["ai"]
    assert "federated_learning" in ai
    assert "privacy_preserving" in ai
    assert "agent_framework" in ai

def test_config_endpoint_unauthorized():
    """Test config endpoint without auth"""
    response = client.get("/config")
    assert response.status_code == 401  # Unauthorized

if __name__ == "__main__":
    pytest.main([__file__])
