pub struct NucleotideCounter {
    pub a: usize,
    pub c: usize,
    pub g: usize,
    pub t: usize,
}

impl NucleotideCounter {
    fn from(a : usize, c : usize, g : usize, t : usize) -> NucleotideCounter {
        Self{a, c, g, t}
    }
}

pub fn counts(dna: &[char]) -> NucleotideCounter {
    
    let mut nucl = NucleotideCounter::from(0, 0, 0, 0);

    for &x in dna {

        match x {
            'A' => nucl.a += 1,
            'C' => nucl.c += 1,
            'G' => nucl.g += 1,
            'T' => nucl.t += 1,
            _ => panic!("Abort! Incorrect dna data"),
        }
    }
    nucl
}

pub fn dna_complement(dna: &[char]) -> Vec<char> {

    let mut v : Vec<char> = Vec::new();

    for &x in dna {

        match x {
            'A' => v.push('T'),
            'C' => v.push('G'),
            'G' => v.push('C'),
            'T' => v.push('A'),
            _ => panic!("Abort! Incorrect dna base!"),
        }
    }
    v
}

pub fn reverse_rna_complement(dna: &[char]) -> Vec<char> {
    let mut v = dna_complement(dna);

    v.iter_mut().for_each(|x| if *x == 'T' {*x = 'U'});
    v.reverse();
    v
    
}