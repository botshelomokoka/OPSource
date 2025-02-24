class StackingPoolMonitor {
  async checkPoolHealth(poolAddress: string) {
    const [lockedSTX, btcAnchor] = await Promise.all([
      stacks.getAccountSTXLocked(poolAddress),
      btc.getChainTipHeight()
    ]);
    
    return {
      reserveRatio: lockedSTX / poolConfig.totalStacked,
      anchorLag: btcAnchor - poolConfig.lastAnchor,
      rewardReadiness: this.calculateRewardProbability(poolAddress)
    };
  }
} 