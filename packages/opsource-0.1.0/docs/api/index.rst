API Reference
=============

Welcome to the OPSource API documentation. Our API allows you to integrate OPSource's powerful features into your own applications.

.. contents:: Table of Contents
   :local:
   :depth: 2

Authentication
-------------

.. code-block:: bash

    curl -X POST https://api.opsource.dev/v1/auth/token \
        -H "Content-Type: application/json" \
        -d '{"api_key": "your_api_key"}'

All API requests must be authenticated using an API key. To get an API key:

1. Log in to your OPSource account
2. Navigate to Settings > API Keys
3. Click "Generate New Key"

.. warning::
   Never share your API key or commit it to version control.

Endpoints
--------

Trading
~~~~~~~

.. http:post:: /v1/trade/order

Create a new trade order.

.. code-block:: json

    {
        "pair": "BTC/USD",
        "side": "buy",
        "amount": 1.0,
        "price": 50000.0,
        "type": "limit"
    }

Analytics
~~~~~~~~

.. http:get:: /v1/analytics/market

Get market analysis for a trading pair.

Parameters:

* ``pair`` (string) - Trading pair (e.g., "BTC/USD")
* ``timeframe`` (string) - Time period (e.g., "1d", "4h", "1w")
* ``indicators`` (array) - List of technical indicators

WebSocket API
------------

Real-time data is available through our WebSocket API:

.. code-block:: javascript

    const ws = new WebSocket('wss://ws.opsource.dev/v1');
    
    ws.onmessage = (event) => {
        const data = JSON.parse(event.data);
        console.log('Received:', data);
    };

Available Streams
~~~~~~~~~~~~~~~

* ``market`` - Real-time market data
* ``trades`` - Live trade updates
* ``orders`` - Order book updates
* ``account`` - Account status changes

Rate Limits
----------

.. list-table::
   :header-rows: 1

   * - API
     - Rate Limit
     - Time Window
   * - REST API
     - 1000 requests
     - Per minute
   * - WebSocket
     - 100 messages
     - Per second

Error Codes
----------

.. code-block:: json

    {
        "error": {
            "code": "INSUFFICIENT_FUNDS",
            "message": "Not enough balance to execute trade",
            "details": {
                "required": 1.0,
                "available": 0.5
            }
        }
    }

Common error codes:

* ``INVALID_AUTH`` - Invalid API key
* ``RATE_LIMIT`` - Rate limit exceeded
* ``INVALID_PARAMS`` - Invalid request parameters
* ``INSUFFICIENT_FUNDS`` - Insufficient balance
* ``ORDER_NOT_FOUND`` - Order ID not found

SDKs & Libraries
--------------

Official SDKs:

* `Python SDK <https://github.com/botshelomokoka/opsource-python>`_
* `JavaScript SDK <https://github.com/botshelomokoka/opsource-js>`_
* `Java SDK <https://github.com/botshelomokoka/opsource-java>`_

Examples
-------

Trading Example
~~~~~~~~~~~~~

.. code-block:: python

    from opsource import Client
    
    client = Client('your_api_key')
    
    # Place a market buy order
    order = client.create_order(
        pair='BTC/USD',
        side='buy',
        amount=1.0,
        type='market'
    )
    
    print(f"Order placed: {order['id']}")

Analytics Example
~~~~~~~~~~~~~~

.. code-block:: python

    # Get market analysis
    analysis = client.get_market_analysis(
        pair='BTC/USD',
        timeframe='1d',
        indicators=['RSI', 'MACD']
    )
    
    print(f"RSI: {analysis['RSI']}")
    print(f"MACD: {analysis['MACD']}")

WebSocket Example
~~~~~~~~~~~~~~

.. code-block:: python

    async def handle_market_data():
        async with client.ws_connect() as ws:
            await ws.subscribe(['market'])
            
            async for msg in ws:
                print(f"Market update: {msg}")

Support
-------

Need help with the API?

* Join our `Discord community <https://discord.gg/opsource>`_
* Email us at api@opsource.dev
* Check our `API status page <https://status.opsource.dev>`_

.. note::
   For enterprise support, please contact enterprise@opsource.dev
