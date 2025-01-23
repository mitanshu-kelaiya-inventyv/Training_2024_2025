use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Hello, welcome to Guess the number!");

    println!("Let me set a number between 1 to 100.");
    let my_no: u8 = rand::thread_rng().gen_range(1..=100);
    println!("{my_no}");

    loop{
    println!("Guess a number: ");
    let mut guess: String= String::new();
    io::stdin().read_line(&mut guess).expect("Please enter valid input!!");
    let guess_no:u8 = match guess.trim().parse(){
        Ok(num) => num,
        Err(message) =>{
            println!("{}", message);
            continue;
        }
    };
    match guess_no.cmp(&my_no) {
        Ordering::Equal => {
            println!("*** Correct guess, the number is {} ***", guess_no);
            break;
        },
        Ordering::Greater =>    println!("Your guess is high."),
        Ordering::Less =>   println!("Your guess is low.")
        
    }
    
    }
    
}
