export class ChannelManager {
  constructor(
    public capacity: number,
    public liquidity: number
  ) {}

  calculateFeeRate(): number {
    return this.liquidity / this.capacity;
  }
} 