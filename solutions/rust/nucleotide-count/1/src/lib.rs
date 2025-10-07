use std::collections::HashMap;

const VALID_NUCLEOTIDES: [char; 4] = ['A', 'T', 'G', 'C'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut count = 0;
    if !VALID_NUCLEOTIDES.contains(&nucleotide) {
        return Err(nucleotide);
    }
    for current in dna.chars() {
        if !VALID_NUCLEOTIDES.contains(&current) {
            return Err(current);
        }
        if current == nucleotide {
            count += 1;
        }
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut result = HashMap::with_capacity(4);
    for nucleotide in VALID_NUCLEOTIDES {
        let result_count = count(nucleotide, dna);
        if result_count.is_ok() {
            result.insert(nucleotide, result_count.unwrap());
        } else {
            return Err(result_count.err().unwrap())
        }
    }
    Ok(result)
}
