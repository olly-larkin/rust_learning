use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if nucleotide != 'A' && nucleotide != 'C' && nucleotide != 'G' && nucleotide != 'T' {
        Err(nucleotide)
    } else {
        match invalid_string(dna) {
            Some(c) => Err(c),
            None => Ok(
                dna.chars()
                    .filter(|&c| c == nucleotide)
                    .count()
            ),
        }
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let char_list = ['A', 'C', 'G', 'T'];
    match invalid_string(dna) {
        Some(c) => Err(c),
        None => Ok(
            char_list.iter()
                .map(|c| *c)
                .zip(
                    char_list.iter()
                        .map(|c| dna.chars().filter(|nuc| c == nuc).count())
                )
                .collect()
        ),
    }
}

fn invalid_string(dna: &str) -> Option<char> {
    let invalid = dna.chars()
        .filter(|&c| c != 'A' && c != 'C' && c != 'G' && c != 'T')
        .collect::<Vec<char>>();
    if invalid.is_empty() {
        None
    } else {
        Some(invalid[0])
    }
}
