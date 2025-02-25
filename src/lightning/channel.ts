export class ChannelManager {
  constructor(
    public totalCapacity: number,
    public availableLiquidity: number
  ) {}

  calculateFeeRate(): number {
    // Based on BIP 341's fee calculation recommendations
    const availableRatio = this.availableLiquidity / this.totalCapacity;
    // Linear fee scaling with minimum 0.25% floor
    return Math.max(0.0025, 0.25 * availableRatio);
  }
} 