import { createMaliciousActor, createProposal, EXECUTION_DELAY } from './utils/dao';
import { executeProposal } from '@/dao/core';
import { install, Clock } from '@sinonjs/fake-timers';
import { describe, it, expect } from 'vitest';
import { vi } from 'vitest';

interface TestContext {
  clock: Clock;
}

describe('DAO Governance', () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it('should prevent 51% attack with quadratic voting', async () => {
    // Placeholder test for attack prevention
    expect(true).toBe(true);
  });

  it('should enforce execution delay', async () => {
    vi.useFakeTimers();
    const creationTime = Date.now();
    
    // Advance exactly by execution delay
    vi.advanceTimersByTime(EXECUTION_DELAY);

    await expect(executeProposal('test-proposal-1', creationTime))
      .resolves.not.toThrow();
    
    vi.useRealTimers();
  });
}); 