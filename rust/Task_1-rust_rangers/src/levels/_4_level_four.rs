use std::io;

pub fn start_level_4(){
    
    println!("Level Four");
    println!("You are in a Dark mine. You are provided with instructuions and you should followthat instructions. If you try anything different here, you dont know what is going to happen, where you will end. So it is better to follow instructions as we are entering a dark mine. Following instructions will take you to a place where you can actually mine, in the next level.");
    let private_key="ABC123";
    println!("Enter your private key to proceed further.");
    let mut input = String::from("");

    io::stdin().read_line(&mut input).expect("Please enter valid input!!");
    
    validate_key(input.trim(), &private_key).expect("Note");
   

    println!("Nice to see that you are following instructions. Here if you dont follow instructions and your cases are not handled explicitly, it will cause program to panic and you will be out of the game. Be careful.");

    println!("")


}

fn validate_key(input:&str, key:&str) -> Result<bool, String>{
    //println!("{input} {key}");
    if  input == key {
        Ok(true)
    }else{
        Err(String::from("Game panicked!!!! Because your key was incorrect and incorrect key input is not explicitly handled in the game, your game panics. See you later"))
    }
}