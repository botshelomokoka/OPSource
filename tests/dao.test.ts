import { getContracts } from '../types/clarinet-sdk';

it('should prevent 51% attack with quadratic voting', async () => {
  const attacker = await createMaliciousActor(51);
  const proposal = await createProposal();
  
  await attacker.voteMultiple(proposal.id, 100);
  await executeWithDelay(proposal.id);
  
  expect(proposal.executed).toBe(false);
  expect(system.vetoCalled).toBe(true);
});

it('should enforce execution delay', async () => {
  await createProposal();
  await time.increase(EXECUTION_DELAY - 60); // 1 minute before
  await expect(executeProposal()).rejects.toThrow("Before execution delay");
  
  await time.increase(120); // 1 minute after
  await executeProposal();
}); 