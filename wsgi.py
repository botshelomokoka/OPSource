from main import app
from fastapi.middleware.wsgi import WSGIMiddleware

# Create WSGI app
application = WSGIMiddleware(app)
