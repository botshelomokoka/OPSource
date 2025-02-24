// Dynamic fee adjustment
const BASE_FEE: u64 = 1000;
const RATE_PER_UNIT_LIQUIDITY: f64 = 0.001;

impl ChannelManager {
    pub fn calculate_routing_fee(&self) -> u64 {
        let liquidity_ratio = self.outbound_capacity as f64 / self.total_capacity as f64;
        let dynamic_fee = (BASE_FEE as f64 * (1.0 + RATE_PER_UNIT_LIQUIDITY / liquidity_ratio)) as u64;
        dynamic_fee.clamp(BASE_FEE, BASE_FEE * 10)
    }

    pub fn update_htlc_limits(&mut self) {
        let max_htlc_value = (self.outbound_capacity as f64 * 0.25) as u64;
        self.config.max_htlc_value = max_htlc_value;
    }
} 