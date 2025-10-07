#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);
const ALLOWED_DNA_NUCLEOTIDES: [char; 4] = ['A', 'T', 'G', 'C'];

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);
const ALLOWED_RNA_NUCLEOTIDES: [char; 4] = ['A', 'U', 'G', 'C'];

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if let Some(mismatch) = dna
            .char_indices()
            .find(|ch| !ALLOWED_DNA_NUCLEOTIDES.contains(&ch.1))
        {
            Err(mismatch.0)
        } else {
            Ok(Dna(dna.to_owned()))
        }
    }

    pub fn into_rna(self) -> Rna {
        let rna_str: String = self.0.chars().map(|nuc| match nuc {
            'G' => 'C',
            'C' => 'G',
            'T' => 'A',
            'A' => 'U',
            _ => panic!("inconceivable")
        })
        .collect();
        
        Rna::new(&rna_str).unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        if let Some(mismatch) = rna
            .chars()
            .enumerate()
            .find(|ch| !ALLOWED_RNA_NUCLEOTIDES.contains(&ch.1))
        {
            Err(mismatch.0)
        } else {
            Ok(Rna(rna.to_owned()))
        }
    }
}
