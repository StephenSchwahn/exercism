type Nucleotide = "A" | "T" | "G" | "C";
function isNucleotide(letter: string): letter is Nucleotide {
  return ["A", "T", "G", "C"].includes(letter);
}

export function nucleotideCounts(DNA: string): Record<Nucleotide, number> {
  const nucleotideArray = Array.from(DNA);
  if (nucleotideArray.some((letter) => !isNucleotide(letter))) {
    throw new Error("Invalid nucleotide in strand");
  }

  return nucleotideArray.reduce<Record<Nucleotide, number>>(
    (
      histogram: Record<Nucleotide, number>,
      letter: string
    ): Record<Nucleotide, number> => {
      // .some() made sure all of these are valid types, but doesnt do type narrowing.
      // faster to assert the type here than mapping all to a type it already is
      histogram[letter as Nucleotide] += 1;
      return histogram;
    },
    { A: 0, T: 0, G: 0, C: 0 }
  );
}