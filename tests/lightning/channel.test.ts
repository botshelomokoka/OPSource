import { describe, it, expect } from 'vitest';
import { ChannelManager } from '@/lightning/core';

describe('Anti-jamming measures', () => {
  it('should adjust fees based on liquidity', () => {
    const channel = new ChannelManager(100_000, 25_000);
    expect(channel.calculateFeeRate()).toBe(0.0001);
  });
}); 