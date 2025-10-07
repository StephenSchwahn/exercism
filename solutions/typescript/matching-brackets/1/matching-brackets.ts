const OPENING_CHARS = ["[", "{", "("] as const;
type OpeningTypes = (typeof OPENING_CHARS)[number];
function isOpeningChar(chr: string): chr is OpeningTypes {
  return OPENING_CHARS.includes(chr as OpeningTypes);
}

const CLOSING_CHARS = ["]", "}", ")"] as const;
type ClosingTypes = (typeof CLOSING_CHARS)[number];
function isClosingChar(chr: string): chr is ClosingTypes {
  return CLOSING_CHARS.includes(chr as ClosingTypes);
}

export function isPaired(input: string): boolean {
  let pairings: Record<OpeningTypes, ClosingTypes> = {
    "(": ")",
    "[": "]",
    "{": "}",
  };
  let stack: ClosingTypes[] = [];

  for (let i = 0; i < input.length; i++) {
    const chr = input[i];
    if (isOpeningChar(chr)) {
      // Push the expected closing bracket upon evaluation
      stack.push(pairings[chr]);
    }
    if (isClosingChar(chr)) {
      const expected = stack.pop();
      if (!expected || expected !== chr) {
        return false;
      }
    }
  }

  // At the end, if the expression is balanced, the stack should be empty.
  return stack.length === 0;
}
