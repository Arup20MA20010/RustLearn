fn main() {
    let mut a = 5;
    println!("Before shadowing {a}");
    a = 8;
    println!("After mutating {a}");
    let a = 6;
    println!("After Shadowing {a}");
    let some_char = '📀';
    println!("{some_char}");
    // compound types in rust
    let tup = (1.2,2,3);
    let (x,y,z) = tup;
    println!("x: {x},y: {y}, z:{z}");

    // array 
    let p = [1,2,3];
    let q = [3;5]; // first is the element second is the length to get 5 3's
    println!("{:?}",p);
    println!("{:?}",q);
}
