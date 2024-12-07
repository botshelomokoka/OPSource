# OPSource Tutorials

Welcome to the OPSource tutorials section! Here you'll find step-by-step guides to help you make the most of the platform.

## Getting Started Tutorials

1. [Basic Setup](basic-setup.md)
2. [First Transaction](first-transaction.md)
3. [Using the Dashboard](using-dashboard.md)

## Advanced Topics

1. [Custom Trading Strategies](trading-strategies.md)
2. [AI Model Integration](ai-integration.md)
3. [Security Best Practices](security-practices.md)

## Video Tutorials

<div class="video-grid">
    <div class="video-card">
        <div class="video-thumbnail">🎥</div>
        <h3>Quick Start Guide</h3>
        <p>Get up and running with OPSource in 10 minutes</p>
        <a href="https://youtube.com/opsource/quickstart" class="button">Watch Now</a>
    </div>
    
    <div class="video-card">
        <div class="video-thumbnail">🎥</div>
        <h3>Advanced Features</h3>
        <p>Deep dive into OPSource's advanced capabilities</p>
        <a href="https://youtube.com/opsource/advanced" class="button">Watch Now</a>
    </div>
</div>

## Interactive Examples

Try out these interactive examples to learn by doing:

<div class="interactive-example">
    <h3>Basic Trade</h3>
    ```rust
    let trade = Trade::new()
        .with_pair("BTC/USD")
        .with_amount(1.0)
        .execute();
    ```
    <button onclick="runExample('basic-trade')">Run Example</button>
</div>

<div class="interactive-example">
    <h3>Market Analysis</h3>
    ```rust
    let analysis = MarketAnalysis::new()
        .with_timeframe("1d")
        .analyze();
    ```
    <button onclick="runExample('market-analysis')">Run Example</button>
</div>

## Hands-on Labs

Work through these practical exercises to gain real-world experience:

1. [Building a Trading Bot](labs/trading-bot.md)
2. [Creating Custom Indicators](labs/custom-indicators.md)
3. [Implementing Risk Management](labs/risk-management.md)

## Community Examples

Check out these examples from our community:

<div class="community-examples">
    <div class="example-card">
        <h3>Automated Trading System</h3>
        <p>By @trader_pro</p>
        <a href="examples/automated-trading.md">View Example</a>
    </div>
    
    <div class="example-card">
        <h3>Custom Analytics Dashboard</h3>
        <p>By @data_wizard</p>
        <a href="examples/analytics-dashboard.md">View Example</a>
    </div>
</div>

## Next Steps

After completing these tutorials, you might want to:

1. [Explore the API Reference](../api/overview.md)
2. [Join Our Community](../community/README.md)
3. [Contribute to OPSource](../contributing/guidelines.md)
