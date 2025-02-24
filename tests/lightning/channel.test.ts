describe('Anti-jamming measures', () => {
  it('should adjust fees based on liquidity', () => {
    const channel = new ChannelManager(100_000, 25_000);
    const fee = channel.calculate_routing_fee();
    expect(fee).toBeGreaterThan(1000);
    expect(fee).toBeLessThan(10000);
  });
}); 