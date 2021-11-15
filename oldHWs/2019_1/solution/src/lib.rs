/// Десетична бройна система: 0-9
pub fn decimal(input: &str) -> Option<u32> {
    unimplemented!()
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

/// Шестнадесетична бройна система: 0-9, последвано от a-f
pub fn hex(input: &str) -> Option<u32> {

    digital_root(input, 16)

    /* let mut sum_digits : u32 = 0;

    let mut chars = input.chars(); // input.chars().next() results in endless loop;
    while let Some(c) = chars.next() {

        match c {
            c if c.is_ascii_hexdigit() => sum_digits = sum_digits + c.to_digit(16).unwrap(),
            _                          => return None,
        }
    }

    if sum_digits < 16 {
        return Some(sum_digits);
    }


    let s = convert_to_hex(sum_digits);
    let ss = &s;
    hex(ss) */
    
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

/// Двоична бройна система: 0-1
pub fn binary(input: &str) -> Option<u32> {
    unimplemented!()
}

