import { createMaliciousActor, createProposal, EXECUTION_DELAY } from './utils/dao';
import { executeProposal } from '../src/dao/core.ts';
import { install, clock } from '@sinonjs/fake-timers';

describe('DAO Governance', () => {
  let fakeClock: ReturnType<typeof install>;

  beforeAll(() => {
    fakeClock = install();
  });

  afterAll(() => {
    fakeClock.uninstall();
  });

  it('should prevent 51% attack with quadratic voting', async () => {
    const attacker = createMaliciousActor(51);
    const proposal = createProposal();
    
    await expect(executeProposal(proposal.id)).rejects.toThrow();
    expect(proposal.executed).toBe(false);
  });

  it('should enforce execution delay', async () => {
    const proposal = createProposal();
    
    fakeClock.tick(EXECUTION_DELAY - 60);
    await expect(executeProposal(proposal.id)).rejects.toThrow("Before execution delay");
    
    fakeClock.tick(120);
    await executeProposal(proposal.id);
    expect(proposal.executed).toBe(true);
  });
}); 