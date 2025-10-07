use std::collections::HashMap;
use std::sync::LazyLock;

/*
 * AUG	Methionine
UUU, UUC	Phenylalanine
UUA, UUG	Leucine
UCU, UCC, UCA, UCG	Serine
UAU, UAC	Tyrosine
UGU, UGC	Cysteine
UGG	Tryptophan
UAA, UAG, UGA	STOP

 */

static PROTEIN_MAP: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| HashMap::from([
    ("AUG", "Methionine"),
    ("UUU", "Phenylalanine"),
    ("UUC", "Phenylalanine"),
    ("UUA", "Leucine"),
    ("UUG", "Leucine"),
    ("UCU", "Serine"),
    ("UCC", "Serine"),
    ("UCA", "Serine"),
    ("UCG", "Serine"),
    ("UAU", "Tyrosine"),
    ("UAC", "Tyrosine"),
    ("UGU", "Cysteine"),
    ("UGC", "Cysteine"),
    ("UGG", "Tryptophan"),
    ("UAA", "STOP"),
    ("UAG", "STOP"),
    ("UGA", "STOP"),
]));



pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let proteins: Vec<Option<&&str>> = rna
        .to_ascii_uppercase()
        .as_bytes()
        .chunks(3)                
        .map(|buf| {
            PROTEIN_MAP.get(str::from_utf8(buf).unwrap_or_default())
        })
        .take_while(|elem| *elem != Some(&"STOP"))
        .collect();
    
    if proteins.contains(&None) {
        None
    } else {
        Some(proteins.iter().map(|protein| *protein.unwrap()).collect())
    }
}
