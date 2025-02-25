Quick Start Guide
================

This guide will help you get started with OPSource quickly.

Prerequisites
------------

* Python 3.8 or higher
* pip package manager
* Git (optional)

Installation
-----------

1. Install via pip:

   .. code-block:: bash

      pip install opsource

2. Configure your environment:

   .. code-block:: bash

      export OPSOURCE_API_KEY="your-api-key"

Basic Usage
----------

Here's a simple example to get you started:

.. code-block:: python

   from opsource import Client

   # Initialize client
   client = Client(api_key="your-api-key")

   # Get market data
   market_data = client.get_market_data("BTC/USD")
   print(market_data)

Next Steps
---------

* Read the :doc:`user-guide/index` for detailed usage instructions
* Check out :doc:`api/index` for API documentation
* Visit :doc:`contributing/index` to contribute to the project
