"""
OPSource Production Server
Uses Waitress WSGI server for production deployment
"""
import logging
from pathlib import Path
from waitress import serve
from main import app
from dev_config import DEV_CONFIG

# Configure logging
logging.basicConfig(
    level=getattr(logging, DEV_CONFIG.logging.level),
    format='%(asctime)s - %(levelname)s - %(message)s',
    filename=str(Path(DEV_CONFIG.logging.dir) / 'server.log'),
    filemode='a'
)
logger = logging.getLogger(__name__)

def start_server():
    """Start the production server using waitress"""
    try:
        host = DEV_CONFIG.api.host
        port = DEV_CONFIG.api.port
        logger.info(f"Starting server on http://{host}:{port}")
        
        serve(
            app,
            host=host,
            port=port,
            threads=DEV_CONFIG.api.workers * 4  # Recommended threads per worker
        )
    except Exception as e:
        logger.error(f"Failed to start server: {str(e)}")
        raise

if __name__ == '__main__':
    start_server()
