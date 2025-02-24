import { Web5 } from '@web5/api';
import { DwnClient } from '@tbd54566975/dwn-sdk-js';
import { IndexedDBStorage } from '@tbd54566975/dwn-sdk-js';

export class AnyaWeb5 {
  private web5: Web5;
  private dwn: DwnClient;

  async init() {
    this.web5 = await Web5.connect();
    this.dwn = new DwnClient({
      did: this.web5.did,
      storage: new IndexedDBStorage()
    });
  }

  async storeMetrics(data: object) {
    const record = await this.dwn.createRecord({
      data: JSON.stringify(data),
      schema: 'anya/metrics/v1'
    });
    await this.dwn.sendRecord(record);
  }
} 