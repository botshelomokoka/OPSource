export class BitcoinWeb5Bridge {
  async signWithDID(psbt: Psbt, did: string) {
    const key = await this.resolveDIDKey(did);
    psbt.signAllInputsHD(key);
    return psbt;
  }

  private async resolveDIDKey(did: string) {
    const resolver = new DidResolver();
    const doc = await resolver.resolve(did);
    return HdKey.fromExtendedKey(doc.verificationMethod[0].publicKeyJwk);
  }
} 