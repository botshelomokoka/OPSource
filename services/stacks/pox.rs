impl PoXOptimizer {
    pub fn calculate_optimal_cycle(
        current_reward: u64,
        btc_fee_rate: SatPerVbyte,
        stx_price: f64
    ) -> u8 {
        let min_cycles = 1;
        let reward_per_cycle = current_reward as f64 / min_cycles as f64;
        let btc_cost = calculate_btc_fee(btc_fee_rate);
        
        (1..=12)
            .map(|cycles| (cycles, reward_per_cycle * cycles as f64))
            .filter(|(_, reward)| *reward > (btc_cost * stx_price))
            .map(|(c, _)| c)
            .min()
            .unwrap_or(min_cycles)
    }
} 