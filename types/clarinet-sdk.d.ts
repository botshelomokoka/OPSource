declare module '@hirosystems/clarinet-sdk' {
  export interface NetworkConfig {
    nodeUrl: string;
    accounts: string[];
  }

  export interface AccountConfig {
    mnemonic: string;
    balance: number;
  }

  export interface ContractConfig {
    source: string;
    dependsOn: string[];
  }

  export function getNetworks(): Record<string, NetworkConfig>;
  export function getAccounts(): Record<string, AccountConfig>;
  export function getContracts(): Record<string, ContractConfig>;
} 