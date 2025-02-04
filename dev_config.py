"""
Development configuration for OPSource AI development environment
Follows Bitcoin Core principles and AI development best practices
"""
from typing import Dict, Any, Optional, List
import os
from pathlib import Path
from dataclasses import dataclass, field
from enum import Enum
import logging

# Base directory
BASE_DIR = Path(__file__).resolve().parent

class NetworkType(Enum):
    MAINNET = 'mainnet'
    TESTNET = 'testnet'
    REGTEST = 'regtest'

@dataclass
class TaprootConfig:
    enabled: bool = True
    schnorr_signatures: bool = True
    asset_issuance: bool = True
    multisig_support: bool = True

@dataclass
class DLCConfig:
    enabled: bool = True
    oracle_verification: bool = True
    privacy_enhanced: bool = True
    max_contract_size: int = 1024  # in bytes

@dataclass
class DWNConfig:
    enabled: bool = True
    port: int = 3002
    protocol_version: str = '0.9'
    interfaces: List[str] = field(default_factory=lambda: ['Collections', 'Messages', 'Records'])
    message_retention_days: int = 30
    max_storage_mb: int = 1024  # 1GB
    encryption_enabled: bool = True
    sync_enabled: bool = True
    sync_interval_minutes: int = 60
    backup_enabled: bool = True
    backup_interval_days: int = 1
    backup_retention_days: int = 30
    permissions_mode: str = 'strict'  # 'strict' or 'permissive'
    auth_required: bool = True

@dataclass
class Web5Config:
    did_enabled: bool = True
    did_method: str = 'ion'  # Default to ION for Bitcoin
    did_resolution_timeout: int = 30  # seconds
    handshake_enabled: bool = True
    handshake_timeout: int = 30  # seconds
    decentralized_storage: bool = True
    storage_endpoint: str = 'http://localhost:3001'
    identity_endpoint: str = 'http://localhost:3002'
    dwn: DWNConfig = field(default_factory=DWNConfig)
    protocol_version: str = '0.9'
    max_message_size_mb: int = 10
    max_concurrent_connections: int = 100
    connection_timeout: int = 30  # seconds
    retry_attempts: int = 3
    retry_delay: int = 5  # seconds

@dataclass
class LightningConfig:
    enabled: bool = True
    port: int = 9735
    rest_port: int = 8080
    watchtower_enabled: bool = True

@dataclass
class RGBConfig:
    enabled: bool = True
    port: int = 3000
    stash_dir: str = str(BASE_DIR / 'rgb_stash')
    taproot_integration: bool = True

@dataclass
class RSKConfig:
    enabled: bool = True
    port: int = 4444
    network: str = 'testnet'
    merge_mining: bool = True

@dataclass
class StacksConfig:
    enabled: bool = True
    port: int = 3999
    api_port: int = 3999
    network: str = 'testnet'

@dataclass
class BitcoinConfig:
    network: NetworkType = NetworkType.TESTNET
    rpc_port: int = 18332
    rpc_user: Optional[str] = None
    rpc_password: Optional[str] = None
    use_auth_cookie: bool = True
    prune: bool = True
    prune_size: int = 5120  # 5GB in MB
    txindex: bool = False
    discover_nodes: bool = True
    max_connections: int = 8
    connect_timeout: int = 30
    taproot: TaprootConfig = field(default_factory=TaprootConfig)
    dlc: DLCConfig = field(default_factory=DLCConfig)
    web5: Web5Config = field(default_factory=Web5Config)
    lightning: LightningConfig = field(default_factory=LightningConfig)
    rgb: RGBConfig = field(default_factory=RGBConfig)
    rsk: RSKConfig = field(default_factory=RSKConfig)
    stacks: StacksConfig = field(default_factory=StacksConfig)

@dataclass
class SecurityConfig:
    enable_ssl: bool = False
    ssl_cert_path: str = str(BASE_DIR / 'certs' / 'dev.crt')
    ssl_key_path: str = str(BASE_DIR / 'certs' / 'dev.key')
    jwt_secret_key: str = os.getenv('JWT_SECRET_KEY', '')
    cors_origins: List[str] = field(default_factory=lambda: ["http://localhost:3000"])
    rate_limit: int = 100
    privacy_preserving: bool = True
    encryption_scheme: str = 'aes-256-gcm'

@dataclass
class AIConfig:
    model_service_port: int = 8001
    inference_service_port: int = 8002
    batch_size: int = 32
    use_gpu: bool = True
    model_cache_dir: str = str(BASE_DIR / 'models')
    federated_learning: bool = True
    privacy_preserving: bool = True
    agent_framework: str = 'autonomous'
    task_management: bool = True

@dataclass
class ApiConfig:
    host: str = '0.0.0.0'
    port: int = 8000
    debug: bool = True
    workers: int = 4

@dataclass
class DevServerConfig:
    port: int = 3000
    hot_reload: bool = True
    watch_files: bool = True

@dataclass
class DatabaseConfig:
    type: str = 'postgresql'
    port: int = 5432
    name: str = 'opsource_dev'
    user: str = os.getenv('DB_USER', 'postgres')
    password: str = os.getenv('DB_PASSWORD', '')
    host: str = 'localhost'
    encryption: bool = True

@dataclass
class LoggingConfig:
    level: str = 'DEBUG'
    dir: str = str(BASE_DIR / 'logs')
    max_size: int = 10485760  # 10MB
    backup_count: int = 5
    include_timestamp: bool = True
    privacy_filter: bool = True

@dataclass
class TestingConfig:
    port: int = 5000
    mock_services: bool = True
    use_regtest: bool = True
    coverage_enabled: bool = True

@dataclass
class DevConfig:
    bitcoin: BitcoinConfig = field(default_factory=BitcoinConfig)
    security: SecurityConfig = field(default_factory=SecurityConfig)
    ai: AIConfig = field(default_factory=AIConfig)
    api: ApiConfig = field(default_factory=ApiConfig)
    dev_server: DevServerConfig = field(default_factory=DevServerConfig)
    database: DatabaseConfig = field(default_factory=DatabaseConfig)
    logging: LoggingConfig = field(default_factory=LoggingConfig)
    testing: TestingConfig = field(default_factory=TestingConfig)

# Development Environment Configuration
DEV_CONFIG = DevConfig()

def load_env_config() -> None:
    """Load environment-specific configuration overrides with security checks"""
    for section, config in vars(DEV_CONFIG).items():
        if isinstance(config, (BitcoinConfig, SecurityConfig, AIConfig, ApiConfig, 
                             DevServerConfig, DatabaseConfig, LoggingConfig, TestingConfig)):
            for key, value in vars(config).items():
                env_key = f'OPSOURCE_{section.upper()}_{key.upper()}'
                if env_key in os.environ:
                    new_value = os.environ[env_key]
                    
                    # Convert to appropriate type
                    if isinstance(value, bool):
                        new_value = new_value.lower() == 'true'
                    elif isinstance(value, int):
                        new_value = int(new_value)
                    elif isinstance(value, float):
                        new_value = float(new_value)
                    
                    setattr(config, key, new_value)

# Load environment overrides with security checks
load_env_config()
