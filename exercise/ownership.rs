
fn main() {

    let mut str : String = String::from("Milan");
    let s2 = str.clone();    

    let arr : [i32; 4] = [1, 3, 5, 7];
    let slice = &arr[2..4];

    let r : &String = &str;
    //r.push_str(" e otbora");

    println!("{}, samo {}", str, str);
    
    println!("{:?}", slice);

    let mut v = vec![2,4,6,8];
    let vr = &mut v;
    vr.push(19);

    println!("{:?}", v);

    println!("{}", length(r));

    take_ownership(str);
    println!("can i take {} back?", r);

}

fn length(s : &String) -> usize {
    s.len()
}

fn take_ownership(s : String) -> () {
    println!("{} is now mine!", s)
}