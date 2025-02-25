export class ChannelManager {
  constructor(
    public capacity: number,
    public liquidity: number
  ) {}

  calculateFeeRate(): number {
    // Available liquidity ratio
    const availableRatio = this.liquidity / this.capacity;
    // Apply 25% multiplier for fee calculation (BIP recommendations)
    return 0.25 * availableRatio;
  }
} 