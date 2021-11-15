
fn main() {

    let s : String = String::from("dd");
    let r = '8';

    println!("{}", None == r.to_digit(8));

    println!("{:?}", octal("345"))

    
    //print!("{:?}", str_to_int(s, 16) == Some(0xdd));

    /* let inp2 = "7b";
    let n : Option<u32> = str_to_int(inp2, 16);
    println!("{:#x}",n.unwrap());
    println!("{}", 15 == 0xf);
    println!("{}", hex("7b") == Some(18));

    let t = u32::from_str_radix(inp2, 16);
    println!("{:?}", t); */
    
}
pub fn convert_to(mut input: u32, radix: u32) -> String {

    let mut s = String::new();
    while input != 0 {
        let digit = input % radix;
        //let ch = (input % radix).to_string();

        if radix == 16 {

            match digit {
                10 => s.push('a'),
                11 => s.push('b'),
                12 => s.push('c'),
                13 => s.push('d'),
                14 => s.push('e'),
                15 => s.push('f'),
                _ => s.push_str(&digit.to_string()),
            }
        }
        else {
            s.push_str(&digit.to_string());
        }
        
        input = input / radix;
    }

    return String::from(s.chars().rev().collect::<String>())
}


/// Осмична бройна система: 0-7
pub fn octal(input: &str) -> Option<u32> {
    //unimplemented!()

    digital_root(input, 8)
}

pub fn digital_root(input: &str, radix: u32) -> Option<u32> {

    let mut sum_digits = 0;
    let mut chars = input.chars();

    while let Some(c) = chars.next() {

        let converted = c.to_digit(radix);

        match converted {
            Some(digit) => sum_digits = sum_digits + digit,
            None        => return None,
        }
    }

    if sum_digits < radix {
        let n = Some(sum_digits);
        n
    }
    else {
        let s = &convert_to(sum_digits, radix);
        digital_root(s, radix)
    }
}

/* pub fn dec_to_hex(mut n : Option<u32>) -> &str {
    
    let s : String = String::new();
    while n != 0 {
        let d : u32 = n % 16;
        s.push(d.)
    }
} */

pub fn hex(input: &str) -> Option<u32> {

    /* let n = str_to_int(input, 16);

    match n {
        None => None,
        Some(num) => Some(digital_root(num, 16))
    } */

    let mut sum_digits : u32 = 0;

    let mut chars = input.chars(); // input.chars().next() results in endless loop;
    while let Some(c) = chars.next() {

        match c {
            c if c.is_ascii_hexdigit() => sum_digits = sum_digits + c.to_digit(16).unwrap(),
            _                          => return None,
        }
    }

    Some(sum_digits)
}

fn str_to_int(input: &str, base: u32) -> Option<u32> {

    let mut n : u32 = 0;
    let mut chars = input.chars(); // input.chars().next() results in endless loop;
    while let Some(c) = chars.next() {
            
        match c {
            //c if base == 16 && c.is_ascii_hexdigit()        => n *= base + c.to_digit(base).unwrap(),
            //c if c.is_digit(base) => n = n * base + c.to_digit(base).unwrap(),
            c if c.is_digit(base) => {
                n = n * base + c.to_digit(base).unwrap();
                println!("{:?}",n);
            }
            _                                  => return None,
        }
    }

    Some(n)
}

fn hex_str_to_int(input: &str) -> Option<u32> {

    let mut n : u32 = 0;
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        
        match c {
            c if c.is_ascii_hexdigit() => n = n * 16 + c.to_digit(16).unwrap(),    
            _                          => return None,
        }
        //println!("{}", c);
    }
    println!("in foo {}", n);
    Some(n)
}
