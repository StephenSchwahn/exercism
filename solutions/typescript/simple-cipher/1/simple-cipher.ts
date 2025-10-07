const CHARACTER_OFFSET = "a".charCodeAt(0);
const DEFAULT_KEY_LENGTH = 100;

export class SimpleCipher {
  private readonly cipherKeyOffsets: number[];

  constructor(maybeCipherKey?: string) {
    if (maybeCipherKey) {
      this.cipherKeyOffsets = maybeCipherKey
        .split("")
        .map((chr: string) => chr.charCodeAt(0) - CHARACTER_OFFSET);
    } else {
      const min = Math.ceil(0);
      const max = Math.floor(25);
      this.cipherKeyOffsets = Array.from(
        { length: DEFAULT_KEY_LENGTH },
        (_, i) => {
          return Math.floor(Math.random() * (max - min + 1)) + min;
        }
      );
    }
  }

  encode(plainText: string) {
    return plainText
      .toLowerCase()
      .split("")
      .map((chr: string, idx: number) => {
        let keyOffset =
          this.cipherKeyOffsets[idx % this.cipherKeyOffsets.length];
        const charNum =
          ((chr.charCodeAt(0) - CHARACTER_OFFSET + keyOffset) % 26) +
          CHARACTER_OFFSET;

        return String.fromCharCode(charNum);
      })
      .join("");
  }

  decode(cipherText: string) {
    return cipherText
      .toLowerCase()
      .split("")
      .map((chr: string, idx: number) => {
        let keyOffset =
          this.cipherKeyOffsets[idx % this.cipherKeyOffsets.length];
        const charNum =
          ((chr.charCodeAt(0) - CHARACTER_OFFSET - keyOffset + 26) % 26) +
          CHARACTER_OFFSET;
        return String.fromCharCode(charNum);
      })
      .join("");
  }

  get key() {
    return this.cipherKeyOffsets
      .map((offset) => String.fromCharCode(offset + CHARACTER_OFFSET))
      .join("");
  }
}
