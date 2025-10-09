// NOTE: Solved this problem with a 0-trust security-minded posture,
// This may not be everyone's solution but it's both pragmatic and safe

// P99 of the latin languages fall under 50 chars.
// thai may be an exception since they use no spaces in their sentences, so
// may tweak for thai and similar languages
const LARGEST_ALLOWABLE_STRING_LENGTH = 50;

export class Anagram {
  sortedInitialString: string;

  constructor(private input: string) {
    if (input.length > LARGEST_ALLOWABLE_STRING_LENGTH) {
      // Don't reveal the upper bound to an attacker - just leave it ambiguous.
      // People that *should* know, can look at the function documentation
      throw new Error("Input String too long. Refusing to compare");
    }

    this.sortedInitialString = Array.from(input.toLowerCase()).sort().join("");
  }

  public matches(...potentials: string[]): string[] {
    // For small n values (string length), the m * O(n log n) is acceptable tradeoff to the O(n) solution
    // that uses maps. The inital startup cost of that O(n) is larger, so at small N values actually
    // takes more time.

    return potentials
      .filter(
        (candidate: string) =>
          // filter out all strings that arent even the same length before doing more expensive operations
          candidate.length === this.input.length
      )
      .filter(
        (candidate: string) =>
          // Next filter out candidates that are the exact same as the initial word.
          candidate.toLowerCase() != this.input.toLowerCase()
      )
      .filter((candidate: string) => {
        const sortedCandidate = Array.from(candidate.toLowerCase()).sort();
        return this.sortedInitialString === sortedCandidate.join("");
      });
  }
}