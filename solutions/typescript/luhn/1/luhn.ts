export function valid(digits: string): boolean {
  if (digits.trim().length < 2) {
    return false
  }

  try {
    return (
      Array.from(digits)
        .filter((digit) => digit != " ")
        .reverse()
        .reduce<number>((checksum, digit, idx) => {
          const numericDigit = parseInt(digit, 10);
          if (Number.isNaN(numericDigit)) {
            throw new Error("Number contains non digit");
          }
          // double every second digit means idx % 2 == 1
          if (idx % 2 === 1) {
            const doubled = numericDigit * 2;
            return checksum + (doubled > 9 ? doubled - 9 : doubled);
          }
          return checksum + numericDigit;
        }, 0) %
        10 ===
      0
    );
  } catch (err: unknown) {
    return false;
  }
}
