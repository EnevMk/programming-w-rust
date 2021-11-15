fn main() {

    /* let rosso = Fan::new("peshe", 2008);
    let r: &mut Fan;
    
    let mut f : Fan = Fan::new("Robi", 2006);
    r = &mut f;

    println!("{:?}", r);

    println!("rossonero {}, fan since {}", rosso.name, rosso.fan_since);

    let r2 : Fan = Fan::from(String::from("Misho"), 2010);

    println!("rossonero {}, fan since {}", r2.name, r2.fan_since); */

    let input: Vec<char> = "GC".chars().collect();

    let c = counts(&input);

    println!("{:?}", c);
}

#[derive(Debug)]
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
    
    for x in dna {

        if x == &'a' {
            nucl.a += 1;
        }

        else if x == &'c' {
            nucl.c += 1;
        }

        else if x == &'g' {
            nucl.g += 1;
        }

        else if x == &'t' {
            nucl.t += 1;
        }

        else {
            panic!("Abort! Incorrect dna base!");
        }
    }

    nucl
}


#[derive(Debug)]
struct Fan {
    name : String,
    fan_since : u32,
}

impl Fan {

    fn new(s: &str, year: u32) -> Fan {

        Fan {
            name : String::from(s),
            fan_since : year,
        }
    }

    fn from(s: String, year : u32) -> Self {

        Fan {
            name : s,
            fan_since : year,
        }
    }
}