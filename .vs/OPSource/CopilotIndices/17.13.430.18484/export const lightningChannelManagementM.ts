export const lightningChannelManagementMock = {
  // ...existing code...
  openChannel: jest.fn((nodeId: string, amount: number) => ({
    channelId: 'mockChannelId',
    nodeId,
    amount,
  })),
  closeChannel: jest.fn((channelId: string) => true),
  getChannelStatus: jest.fn((channelId: string) => ({
    channelId,
    status: 'open',
  })),
  listChannels: jest.fn(() => [
    { channelId: 'mockChannelId1', nodeId: 'node1', amount: 1000 },
    { channelId: 'mockChannelId2', nodeId: 'node2', amount: 2000 },
  ]),
  // ...existing code...
};
