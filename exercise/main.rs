//use solution::*;

fn main() {

    let msg = String::from("Milan Torino Ibra");

    let res = first_word(&msg);

    println!("{}", res);

    let ip : IpAddr = IpAddr::V4(127, 0, 0, 1);
    let home : IpAddr = IpAddr::V6(String::from("::1"))
    println!("{:?}", ip);
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}



fn first_word(s: &String) -> String {
    
    let mut i = 0;

    for ch in s.chars() {
        if ch == ' ' {
            break;
        }
        i += 1;
    }
    
    let slice = &s[0..i];
    let st : String = String::from(slice);
    st
}