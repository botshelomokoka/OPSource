export function createMaliciousActor(percentage: number) {
  return {
    address: `0xmalicious${percentage}`,
    votingPower: BigInt(percentage * 1e6)
  };
}

export function createProposal() {
  return {
    id: crypto.randomUUID(),
    executed: false
  };
}

export const EXECUTION_DELAY = 172800; // 2 days in seconds 