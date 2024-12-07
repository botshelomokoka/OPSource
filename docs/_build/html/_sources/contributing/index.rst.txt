Contributing Guide
================

Thank you for your interest in contributing to OPSource! This guide will help you get started with contributing to our project.

.. contents:: Table of Contents
   :local:
   :depth: 2

Code of Conduct
-------------

We are committed to providing a friendly, safe, and welcoming environment for all contributors. Please read and follow our :doc:`code-of-conduct`.

Getting Started
-------------

1. Fork the repository
2. Clone your fork:

   .. code-block:: bash

       git clone https://github.com/your-username/opsource.git
       cd opsource

3. Create a virtual environment:

   .. code-block:: bash

       python -m venv venv
       source venv/bin/activate  # Linux/macOS
       venv\Scripts\activate     # Windows

4. Install dependencies:

   .. code-block:: bash

       pip install -r requirements.txt
       pip install -r requirements-dev.txt

Development Process
----------------

1. Create a branch:

   .. code-block:: bash

       git checkout -b feature/your-feature-name

2. Make your changes
3. Run tests:

   .. code-block:: bash

       pytest tests/
       cargo test    # for Rust components

4. Submit a pull request

Coding Standards
--------------

Python
~~~~~~

We follow PEP 8 with these additions:

.. code-block:: python

    # Good
    def calculate_position(price: float, amount: float) -> float:
        """Calculate position value.
        
        Args:
            price: Asset price
            amount: Position size
            
        Returns:
            float: Position value
        """
        return price * amount

Rust
~~~~

Follow the Rust style guide:

.. code-block:: rust

    // Good
    pub struct Position {
        price: f64,
        amount: f64,
    }
    
    impl Position {
        pub fn new(price: f64, amount: f64) -> Self {
            Self { price, amount }
        }
    }

Documentation
-----------

* Use clear, concise language
* Include code examples
* Add docstrings to all public APIs
* Update relevant documentation files

Testing
------

Requirements:

* Unit tests for all new features
* Integration tests for API changes
* Performance tests for critical paths
* Documentation tests

.. code-block:: python

    # test_trading.py
    def test_order_creation():
        order = create_order(
            pair="BTC/USD",
            side="buy",
            amount=1.0
        )
        assert order.status == "pending"
        assert order.pair == "BTC/USD"

Pull Request Process
-----------------

1. Update documentation
2. Add tests
3. Update CHANGELOG.md
4. Get review from maintainers
5. Address feedback
6. Merge after approval

.. note::
   PRs must pass all CI checks before merging.

Issue Reporting
-------------

When reporting issues:

1. Use issue templates
2. Include reproduction steps
3. Attach relevant logs
4. Add system information

Example:

.. code-block:: text

    ## Description
    Trading order fails to execute
    
    ## Steps to Reproduce
    1. Create buy order
    2. Set price at market
    3. Submit order
    
    ## Expected Behavior
    Order executes successfully
    
    ## Actual Behavior
    Order fails with timeout
    
    ## System Info
    - OS: Windows 11
    - Python: 3.11
    - OPSource: 1.0.0

Release Process
------------

1. Version bump
2. Update CHANGELOG.md
3. Create release branch
4. Run release tests
5. Create GitHub release
6. Deploy to production

Security
-------

Report security issues to security@opsource.dev

DO NOT:
* Create public issues
* Share exploit details
* Disclose vulnerabilities

Community
--------

Join our community:

* `Discord <https://discord.gg/opsource>`_
* `Twitter <https://twitter.com/opsource>`_
* `Forum <https://forum.opsource.dev>`_

Recognition
---------

Contributors will be:

* Added to CONTRIBUTORS.md
* Mentioned in release notes
* Invited to core team (significant contributions)

.. grid:: 2

    .. grid-item-card:: Core Team
        :link: team/core
        :link-type: doc

        Meet our core development team.

    .. grid-item-card:: Contributors
        :link: team/contributors
        :link-type: doc

        View all project contributors.

License
------

By contributing, you agree to license your work under our project license (MIT).
