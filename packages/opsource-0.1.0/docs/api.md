# OPSource API Documentation

## Overview

The OPSource API provides programmatic access to blockchain analytics and machine learning capabilities.

## Authentication

All API endpoints require authentication using an API key. You can obtain your API key from the dashboard.

```python
headers = {
    'Authorization': 'Bearer YOUR_API_KEY',
    'Content-Type': 'application/json'
}
```

## Endpoints

### Analytics API

#### Get Transaction Analysis
```http
GET /api/v1/analytics/transaction/{tx_hash}
```

#### Get Wallet Analysis
```http
GET /api/v1/analytics/wallet/{wallet_address}
```

### Machine Learning API

#### Get Prediction
```http
POST /api/v1/ml/predict
```

#### Train Custom Model
```http
POST /api/v1/ml/train
```

## Rate Limits

- Free tier: 1000 requests per day
- Pro tier: 10000 requests per day
- Enterprise tier: Custom limits

## Error Codes

- 200: Success
- 400: Bad Request
- 401: Unauthorized
- 403: Forbidden
- 429: Too Many Requests
- 500: Internal Server Error

## Support

For API support, please contact:
- Email: api@opsource.com
- Discord: [OPSource Discord](https://discord.gg/opsource)
