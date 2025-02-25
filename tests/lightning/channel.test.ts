import { describe, it, expect } from 'vitest';
import { ChannelManager } from '@/lightning/core';

describe('Anti-jamming measures', () => {
  it('should adjust fees based on liquidity', () => {
    const channel = new ChannelManager(100_000, 25_000);
    
    // 25% available liquidity = 0.25 * 0.25 = 0.0625 (6.25%)
    expect(channel.calculateFeeRate()).toBeCloseTo(0.0625, 4);
  });
}); 