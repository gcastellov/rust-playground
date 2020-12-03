use std::collections::HashMap;

const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !NUCLEOTIDES.contains(&nucleotide) {
        return Err(nucleotide);
    }

    match nucleotide_counts(dna) {
        Ok(char_map) => {
            let mut nucleotide_count = 0;
            if let Some(c) = char_map.get(&nucleotide) {
                nucleotide_count = *c;
            }

            Ok(nucleotide_count)
        }
        Err(c) => Err(c),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut char_map: HashMap<char, usize> = NUCLEOTIDES
        .iter()
        .map(|c| (*c, 0))
        .collect::<HashMap<char, usize>>();

    for c in dna.chars() {
        if !NUCLEOTIDES.contains(&c) {
            return Err(c);
        }

        *char_map.entry(c).or_insert(0) += 1;
    }

    Ok(char_map)
}
