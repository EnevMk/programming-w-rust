
fn main() {

    //let arr = [1, 2, 3, 4];
    /* let one : i32 = 1;
    let mut avg = (5 / 3) as f32;
    let _test = one as f32 + 0.5;
    avg = avg + 0.5;
    println!("{}", avg);

    let mut msg : String = String::from("Jigula");
    println!("{}", msg);

    greet(msg); */
    /* modify(&mut msg);

    //msg = String::from("Golf II");
    println!("{}", msg); */
    
    let mut s = String::from("Jigula");
    
    // let s = String::from("Mazda 6");

    change_ref(&mut s);

    println!("{}", s);

    greet(&s);

    println!("s is still mine!");

    let arr : [i32; 8] = [1,2, 3, 4, 5, 6, 7, 8];

    

    println!("{:?}", &arr[1..7]);
}

fn change_ref(name : &mut String) {
    name.clear();
    name.push_str("Gouf 3ka");
}

fn change_borrow(mut name : String) {
    name.clear();
    name.push_str("mini");
    
    println!("Hello, {}", name)
}

fn greet(name : &String) {
    /* name.clear();
    name.push_str("mini"); */
    println!("Hello, {}", name)
}

/* fn fib(a: u32) -> u32 {

    fib_helper(a, 0, 1)
}

fn fib_helper(i: u32, a: u32, b: u32) -> u32 {
    if i == 0 {return a;}
    
    if i == 1 {
        return b;
    }

    fib_helper(i - 1, b, a + b)
} */