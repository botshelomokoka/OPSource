import pytest
from fastapi.testclient import TestClient
from dash33 import create_app

@pytest.fixture
def client():
    app = create_app()
    return TestClient(app)

def test_dashboard_health(client):
    response = client.get("/api/v1/health")
    assert response.status_code == 200
    assert response.json() == {"status": "healthy"}

def test_wallet_connection(client):
    wallet_id = "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx"
    response = client.post(f"/api/v1/wallet/connect/{wallet_id}")
    assert response.status_code in [200, 400]  # 400 if wallet invalid 