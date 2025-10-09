export function valid(digits: string): boolean {
  const filteredDigits = Array.from(digits).filter(digit => digit != " ");
  if (filteredDigits.length < 2) {
    return false;
  }

  const luhnResult = 
    filteredDigits
    .reverse()
    .reduce<number>((checksum, digit, idx) => {
      const numericDigit = parseInt(digit, 10);
      if (idx % 2 === 1) {
        const doubled = numericDigit * 2;
        return checksum + (doubled > 9 ? doubled - 9 : doubled);
      }
      return checksum + numericDigit;
    }, 0);

  return luhnResult % 10 === 0;
}
