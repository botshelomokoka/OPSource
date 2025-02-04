"""
Setup script for OPSource development environment
Implements Bitcoin Core principles and AI development best practices
"""
import os
import sys
import subprocess
import shutil
from pathlib import Path
from typing import Optional, List, Dict
import json
import logging
from datetime import datetime

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

class DevEnvironment:
    def __init__(self):
        self.base_dir = Path(__file__).resolve().parent
        self.venv_dir = self.base_dir / '.venv'
        self.requirements_file = self.base_dir / 'requirements.txt'
        self.repo_url = "https://github.com/botshelomokoka/OPSource.git"
        
        # Essential directories
        self.directories = {
            'logs': self.base_dir / 'logs',
            'certs': self.base_dir / 'certs',
            'models': self.base_dir / 'models',
            'data': self.base_dir / 'data',
            'rgb_stash': self.base_dir / 'rgb_stash',
            'tests': self.base_dir / 'tests',
            'web5': self.base_dir / 'web5',
            'dlc': self.base_dir / 'dlc',
            'taproot': self.base_dir / 'taproot',
            'identity': self.base_dir / 'identity',
            'bitcoin': self.base_dir / 'bitcoin'
        }
        
        # Service ports configuration
        self.ports = {
            'API Server': 8000,
            'Model Service': 8001,
            'Inference Service': 8002,
            'Development Server': 3000,
            'Bitcoin RPC (testnet)': 18332,
            'Lightning Network': 9735,
            'RGB Node': 3000,
            'RSK Node': 4444,
            'Stacks Node': 3999,
            'Web5 Identity': 3001,
            'Web5 Storage': 3002,
            'Database': 5432,
            'Test Server': 5000
        }
        
        # Critical packages with version requirements
        self.critical_packages = {
            'python-bitcoinlib': '>=0.11.0',
            'bitcoin-utils': '>=0.5.3',
            'tensorflow': '>=2.13.0',
            'torch': '>=2.0.0',
            'web3': '>=6.0.0',
            'fastapi': '>=0.100.0',
            'did-resolver': '>=2.0.0',
            'ion-sdk-python': '>=0.5.0',
            'handshake-api': '>=1.0.0',
            'taproot-utils': '>=0.1.0',
            'dlc-protocol': '>=0.1.0'
        }

    def create_directories(self) -> None:
        """Create necessary directories with proper permissions"""
        for dir_name, dir_path in self.directories.items():
            try:
                os.makedirs(dir_path, exist_ok=True)
                logger.info(f"Created directory: {dir_name}")
            except Exception as e:
                logger.error(f"Error creating directory {dir_name}: {str(e)}")
                raise

    def setup_virtual_environment(self) -> None:
        """Create and configure virtual environment"""
        if not self.venv_dir.exists():
            try:
                subprocess.run([sys.executable, '-m', 'venv', str(self.venv_dir)], check=True)
                logger.info("Created virtual environment")
            except subprocess.CalledProcessError as e:
                logger.error(f"Error creating virtual environment: {str(e)}")
                raise

    def get_pip_path(self) -> Path:
        """Get the pip path in the virtual environment"""
        if sys.platform == 'win32':
            return self.venv_dir / 'Scripts' / 'pip.exe'
        return self.venv_dir / 'bin' / 'pip'

    def install_requirements(self) -> None:
        """Install Python dependencies with error handling"""
        pip_path = self.get_pip_path()
        try:
            # Upgrade pip first
            subprocess.run([str(pip_path), 'install', '--upgrade', 'pip'], check=True)
            
            # Install requirements
            subprocess.run([str(pip_path), 'install', '-r', str(self.requirements_file)], check=True)
            logger.info("Installed all requirements successfully")
        except subprocess.CalledProcessError as e:
            logger.error(f"Error installing requirements: {str(e)}")
            raise

    def verify_installation(self) -> Dict[str, bool]:
        """Verify critical components are installed correctly"""
        verification = {}
        pip_path = self.get_pip_path()
        
        try:
            # Get list of installed packages
            result = subprocess.run(
                [str(pip_path), 'freeze'],
                capture_output=True,
                text=True,
                check=True
            )
            installed_packages = result.stdout.lower()
            
            # Verify critical packages
            for package, version in self.critical_packages.items():
                verification[f"{package} {version}"] = package.lower() in installed_packages
            
            return verification
        except subprocess.CalledProcessError as e:
            logger.error(f"Error verifying installation: {str(e)}")
            raise

    def clone_repository(self) -> None:
        """Clone the OPSource repository"""
        try:
            if not (self.base_dir / '.git').exists():
                logger.info(f"Cloning repository from {self.repo_url}")
                subprocess.run(['git', 'clone', self.repo_url, str(self.base_dir)], check=True)
                logger.info("Repository cloned successfully")
            else:
                logger.info("Repository already exists, pulling latest changes")
                subprocess.run(['git', 'pull'], cwd=str(self.base_dir), check=True)
        except subprocess.CalledProcessError as e:
            logger.error(f"Error cloning/pulling repository: {str(e)}")
            raise

    def setup_bitcoin_node(self) -> None:
        """Configure Bitcoin node with pruning enabled"""
        try:
            bitcoin_dir = self.directories['bitcoin']
            bitcoin_conf = bitcoin_dir / 'bitcoin.conf'
            
            # Create bitcoin directory if it doesn't exist
            os.makedirs(bitcoin_dir, exist_ok=True)
            
            # Basic Bitcoin configuration with pruning
            config = [
                "# Bitcoin Core Configuration",
                "testnet=1",
                "prune=5120",  # 5GB pruning
                "maxconnections=8",
                "discover=1",
                "timeout=30",
                "rpcallowip=127.0.0.1",
                "rpcbind=127.0.0.1",
                "server=1",
                "rest=1",
                "daemon=1",
                "deprecatedrpc=signrawtransaction",
                f"datadir={bitcoin_dir}",
                "",
                "# Network",
                "listen=1",
                "upnp=1",
                "dns=1",
                "",
                "# Memory Pool",
                "maxmempool=100",
                "mempoolexpiry=72",
                "",
                "# Wallet",
                "disablewallet=0",
                "keypool=1000",
                "",
                "# ZMQ",
                "zmqpubrawblock=tcp://127.0.0.1:28332",
                "zmqpubrawtx=tcp://127.0.0.1:28333",
                "",
                "# RPC",
                "rpcuser=opsource",
                "rpcpassword=opsource123",  # This should be changed in production
                "rpcport=18332",
            ]
            
            # Write configuration
            with open(bitcoin_conf, 'w') as f:
                f.write('\n'.join(config))
            
            logger.info("Bitcoin node configuration created successfully")
            logger.info("Note: Please change the RPC credentials in bitcoin.conf before running in production")
            
        except Exception as e:
            logger.error(f"Error setting up Bitcoin node: {str(e)}")
            raise

    def setup_web5_environment(self) -> None:
        """Setup Web5-specific components"""
        try:
            # Create Web5 configuration
            web5_config = {
                "did": {
                    "method": "ion",
                    "network": "testnet"
                },
                "identity": {
                    "endpoint": "http://localhost:3001"
                },
                "storage": {
                    "endpoint": "http://localhost:3002"
                }
            }
            
            # Save Web5 configuration
            web5_config_path = self.directories['web5'] / 'config.json'
            with open(web5_config_path, 'w') as f:
                json.dump(web5_config, f, indent=2)
            
            logger.info("Web5 environment configured successfully")
        except Exception as e:
            logger.error(f"Error setting up Web5 environment: {str(e)}")
            raise

    def setup_environment(self) -> None:
        """Set up the complete development environment"""
        try:
            # Clone repository
            self.clone_repository()
            
            # Create directory structure
            self.create_directories()
            
            # Setup virtual environment
            self.setup_virtual_environment()
            
            # Install requirements
            self.install_requirements()
            
            # Setup Bitcoin node
            self.setup_bitcoin_node()
            
            # Setup Web5 environment
            self.setup_web5_environment()
            
            # Verify installation
            verification = self.verify_installation()
            
            # Log verification results
            for package, installed in verification.items():
                status = "✓ Installed" if installed else "✗ Missing"
                logger.info(f"{package}: {status}")
            
            # Display configuration
            self.display_configuration()
            
        except Exception as e:
            logger.error(f"Setup failed: {str(e)}")
            raise

    def display_configuration(self) -> None:
        """Display the development environment configuration"""
        logger.info("\nDevelopment Environment Configuration:")
        logger.info("=====================================")
        
        logger.info("\nAvailable Services:")
        for service, port in self.ports.items():
            logger.info(f"- {service}: {port}")
        
        logger.info("\nDirectory Structure:")
        for dir_name, dir_path in self.directories.items():
            logger.info(f"- {dir_name}: {dir_path}")
        
        logger.info("\nWeb5 Components:")
        logger.info("- DID Method: ION")
        logger.info("- Identity Service: http://localhost:3001")
        logger.info("- Storage Service: http://localhost:3002")

def main():
    """Main entry point for setup"""
    try:
        env = DevEnvironment()
        env.setup_environment()
    except Exception as e:
        logger.error(f"Setup failed: {str(e)}")
        sys.exit(1)

if __name__ == '__main__':
    main()
