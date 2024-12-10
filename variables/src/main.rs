fn main() {
    let mut a = 5;
    println!("Before shadowing {a}");
    a = 8;
    println!("After mutating {a}");
    let a = 6;
    println!("After Shadowing {a}");
}
