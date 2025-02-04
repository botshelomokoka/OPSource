"""
OPSource API Server
Implements Bitcoin Core principles and AI development best practices
"""
from fastapi import FastAPI, HTTPException, Depends, Request
from fastapi.middleware.cors import CORSMiddleware
from fastapi.security import OAuth2PasswordBearer
from typing import Dict, Any, Optional
import logging
from pathlib import Path
from dev_config import DEV_CONFIG

# Configure logging
logging.basicConfig(
    level=getattr(logging, DEV_CONFIG.logging.level),
    format='%(asctime)s - %(levelname)s - %(message)s',
    filename=str(Path(DEV_CONFIG.logging.dir) / 'api.log'),
    filemode='a'
)
logger = logging.getLogger(__name__)

# Initialize FastAPI app
app = FastAPI(
    title="OPSource API",
    description="Bitcoin Core and AI Development API",
    version="1.0.0"
)

# Configure CORS
app.add_middleware(
    CORSMiddleware,
    allow_origins=DEV_CONFIG.security.cors_origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Security
oauth2_scheme = OAuth2PasswordBearer(tokenUrl="token")

@app.get("/")
async def root():
    """Root endpoint"""
    return {
        "name": "OPSource API",
        "version": "1.0.0",
        "status": "operational",
        "bitcoin_network": DEV_CONFIG.bitcoin.network.value
    }

@app.get("/health")
async def health_check():
    """Health check endpoint"""
    return {
        "status": "healthy",
        "components": {
            "bitcoin": {
                "network": DEV_CONFIG.bitcoin.network.value,
                "pruned": DEV_CONFIG.bitcoin.prune,
                "prune_size": f"{DEV_CONFIG.bitcoin.prune_size}MB" if DEV_CONFIG.bitcoin.prune else "N/A",
                "taproot": {
                    "enabled": DEV_CONFIG.bitcoin.taproot.enabled,
                    "schnorr_signatures": DEV_CONFIG.bitcoin.taproot.schnorr_signatures
                },
                "dlc": {
                    "enabled": DEV_CONFIG.bitcoin.dlc.enabled,
                    "privacy_enhanced": DEV_CONFIG.bitcoin.dlc.privacy_enhanced
                }
            },
            "web5": {
                "did_enabled": DEV_CONFIG.bitcoin.web5.did_enabled,
                "handshake_enabled": DEV_CONFIG.bitcoin.web5.handshake_enabled,
                "decentralized_storage": DEV_CONFIG.bitcoin.web5.decentralized_storage
            },
            "layer2": {
                "lightning": {
                    "enabled": DEV_CONFIG.bitcoin.lightning.enabled,
                    "watchtower": DEV_CONFIG.bitcoin.lightning.watchtower_enabled
                },
                "rgb": {
                    "enabled": DEV_CONFIG.bitcoin.rgb.enabled,
                    "taproot_integration": DEV_CONFIG.bitcoin.rgb.taproot_integration
                },
                "rsk": {
                    "enabled": DEV_CONFIG.bitcoin.rsk.enabled,
                    "merge_mining": DEV_CONFIG.bitcoin.rsk.merge_mining
                },
                "stacks": {
                    "enabled": DEV_CONFIG.bitcoin.stacks.enabled,
                    "network": DEV_CONFIG.bitcoin.stacks.network
                }
            },
            "ai": {
                "federated_learning": DEV_CONFIG.ai.federated_learning,
                "privacy_preserving": DEV_CONFIG.ai.privacy_preserving,
                "agent_framework": DEV_CONFIG.ai.agent_framework
            }
        }
    }

@app.get("/config")
async def get_config(token: str = Depends(oauth2_scheme)):
    """Get configuration (requires authentication)"""
    try:
        # Remove sensitive information
        safe_config = {
            "bitcoin": {
                "network": DEV_CONFIG.bitcoin.network.value,
                "prune": DEV_CONFIG.bitcoin.prune,
                "prune_size": DEV_CONFIG.bitcoin.prune_size,
                "discover_nodes": DEV_CONFIG.bitcoin.discover_nodes,
                "max_connections": DEV_CONFIG.bitcoin.max_connections,
                "taproot": {
                    "enabled": DEV_CONFIG.bitcoin.taproot.enabled,
                    "asset_issuance": DEV_CONFIG.bitcoin.taproot.asset_issuance
                }
            },
            "web5": {
                "did_enabled": DEV_CONFIG.bitcoin.web5.did_enabled,
                "handshake_enabled": DEV_CONFIG.bitcoin.web5.handshake_enabled,
                "decentralized_storage": DEV_CONFIG.bitcoin.web5.decentralized_storage
            },
            "ai": {
                "federated_learning": DEV_CONFIG.ai.federated_learning,
                "privacy_preserving": DEV_CONFIG.ai.privacy_preserving,
                "agent_framework": DEV_CONFIG.ai.agent_framework,
                "task_management": DEV_CONFIG.ai.task_management
            }
        }
        return safe_config
    except Exception as e:
        logger.error(f"Error retrieving configuration: {str(e)}")
        raise HTTPException(status_code=500, detail="Internal server error")

# Error handlers
@app.exception_handler(HTTPException)
async def http_exception_handler(request: Request, exc: HTTPException):
    """Handle HTTP exceptions"""
    return {
        "status": "error",
        "code": exc.status_code,
        "message": exc.detail
    }

@app.exception_handler(Exception)
async def general_exception_handler(request: Request, exc: Exception):
    """Handle general exceptions"""
    logger.error(f"Unhandled exception: {str(exc)}")
    return {
        "status": "error",
        "code": 500,
        "message": "Internal server error"
    }

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(
        "main:app",
        host=DEV_CONFIG.api.host,
        port=DEV_CONFIG.api.port,
        reload=DEV_CONFIG.dev_server.hot_reload,
        workers=DEV_CONFIG.api.workers
    )
