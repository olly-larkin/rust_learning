#[derive(Debug, PartialEq)]
pub struct DNA {
    dna: String,
}

impl DNA {
    const VALID: &'static str = "ACGT";
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    rna: String,
}

impl RNA {
    const VALID: &'static str = "ACGU";
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        for (i, c) in dna.chars().enumerate() {
            if DNA::VALID.find(c) == None {
                return Err(i);
            }
        }
        Ok(DNA{ dna: dna.to_string() })
    }

    pub fn into_rna(self) -> RNA {
        RNA::new(
            self.dna.chars()
                .map(|c| match c {
                    'A' => 'U',
                    'C' => 'G',
                    'G' => 'C',
                    'T' => 'A',
                    _ => c,
                })
                .collect::<String>()
                .as_str()
        ).unwrap()
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        for (i, c) in rna.chars().enumerate() {
            if RNA::VALID.find(c) == None {
                return Err(i);
            }
        }
        Ok(RNA{ rna: rna.to_string() })
    }
}
