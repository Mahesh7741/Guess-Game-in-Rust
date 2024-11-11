use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please Enter your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess_number: u32 =match  guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Please enter a vaild number");
                continue;
            }
        };
        println!("You guessed: {guess_number}");

        match guess_number.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
            Ordering::Greater => println!("Too Greater!"),
            Ordering::Less => println!("Too Low!"),
        }
    }
}
