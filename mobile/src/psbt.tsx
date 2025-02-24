import {LDKMobile} from 'react-native-ldk';
import {HSMKeystore} from './hsm';

export class PSBTManager {
  async coSignPSBT(psbtBase64: string): Promise<string> {
    const psbt = await LDKMobile.parsePSBT(psbtBase64);
    const requiredSigs = HSMKeystore.getRequiredSignatures(psbt);
    const signedTx = await HSMKeystore.signTransaction(psbt);
    return signedTx.toBase64();
  }
} 