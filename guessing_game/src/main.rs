use rand::Rng;
use std::io;
use std::cmp::Ordering;
fn main() {

    let secret = rand::thread_rng().gen_range(1..=100);
    loop{
        let mut guess = String::new();

        println!("Please enter the guess");
        io::stdin().read_line(&mut guess).
            expect("Unable to take the input");



        let guess:u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Enter a number");
                continue;
            },
        };

        match guess.cmp(&secret){
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("Win");
                break;
            }

        }
        
    }
}
