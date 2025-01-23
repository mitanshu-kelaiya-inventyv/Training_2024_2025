use std::io;
use tokio::time::{sleep, Duration};

#[tokio::main]
pub async fn start_level_2() {
    println!("\n\n\nWelcome to Level Two, where we find ourselves near the enchanting Deep Blue River. Here, we have three  boats, each named after a fundamental concept in the Rust programming language.\n\n1)There's the boat named 'For' which rows with precision and efficiency, much like a for loop that iterates through elements seamlessly.\n2)We have the 'While' boat, known for its ability to maintain momentum as it glides along the river, reminiscent of a while loop that continues until its condition is no longer met.\n3)We come to the 'Loop' boat, a true marvel that embodies the essence of perpetual motion, just like a loop that cycles endlessly.");

    println!("\nThere are 5 checkpoints in the river, which are spots for some activities. You can avoid activity by pressing 'continue' button which is 'C' press key. Note that if you are sailing in 'loop' boat, be careful because unlike 'for' and 'while' boats, there is no condition set to stop your boat at next level. If you fail to break, then your boat may panic and you will be out of the game.");

    const CHECKS: [&str; 5] = [
        "Scenic Spot",
        "Wildlife Watching",
        "Fishing Spot",
        "Small Island",
        "The Deep Bridge",
    ];

    let mut activities: [bool; 5] = [false; 5];
    let mut input: String = String::from("");
    let mut ind=0;
    println!(
        "Let us pre-configure our boats. Press 'Y' or 'y' for activity and 'c' or 'C' to skip it."
    );
    loop {
        println!("Chekpoint {ind}: {}",CHECKS[ind]);
        input.clear();
        io::stdin().read_line(&mut input).expect("Invalid input");

        match input.trim().to_lowercase().as_str(){
            "y"=>{
                activities[ind]=true;
                ind+=1;
            },
            "c"=>{
                activities[ind]=false;
                ind+=1;
            },
            _=>{
                println!("Invalid button!!");
            }
        }

        if ind == 5{
            break;
        }
    }
    println!("Choose your boat: ");
    input.clear();
    io::stdin().read_line(&mut input).expect("Invalid input");

    match input.trim().to_lowercase().as_str() {
        "for" => {
            println!("Boat configuring!!");
            sleep(Duration::from_secs(2)).await;
            println!("Good to go.");
            for_boat(&CHECKS, &activities).await;
        }
        "while" => {
            println!("Boat configuring!!");
            sleep(Duration::from_secs(2)).await;
            println!("Good to go.");
            while_boat(&CHECKS, &activities).await;
        }
        "loop" => {
            println!("Boat configuring!!");
            sleep(Duration::from_secs(2)).await;
            println!("Good to go.");
            loop_boat(&CHECKS, &activities).await;
        }
        _ => {
            print!("Sorry! Because no boat choosen, you cant move ahead.")
        }
    }

    println!("Woohoooo. You are now onto the level 3. Hope you enjoyed rowing over the river.");
}

async fn for_boat(checks: &[&str; 5], activities:&[bool;5]) {
    for (checkpoint, activity) in checks.iter().zip(activities.iter()) {
        if !activity{
        println!("Upcoming spot: {checkpoint}");
        sleep(Duration::from_secs(2)).await;
            continue;
        }

        println!("Upcoming spot: {checkpoint}");
        sleep(Duration::from_secs(2)).await;
        println!("Current spot: {checkpoint}.");
        sleep(Duration::from_secs(5)).await;
    }
}

async fn while_boat(checks: &[&str; 5], activities:&[bool;5]) {
    let mut size: usize = 0;
    while size < checks.len() {
        if !activities[size]{
        println!("Upcoming spot: {}", checks[size]);
        sleep(Duration::from_secs(2)).await;
            size+=1;
            continue;
        }
        println!("Upcoming spot: {}", checks[size]);
        sleep(Duration::from_secs(2)).await;
        println!("Current spot: {}.", checks[size]);
        sleep(Duration::from_secs(5)).await;
        size += 1;
    }
}

async fn loop_boat(checks: &[&str; 5], activities:&[bool;5]) {
    let mut ind: usize = 0;
    
    loop {
        if ind == 5 {
            println!("Please press breaks by typing 'break' else your game may panic");
            let mut input: String = String::from("");
            input.clear();
            io::stdin().read_line(&mut input).expect("Invalid input");

            match input.trim() {
                "break" => {
                    break;
                }
                _ => {}
            }
        }

        if !activities[ind]{
         println!("Upcoming spot: {}", checks[ind]);
        sleep(Duration::from_secs(2)).await;
            ind+=1;
            continue;
        }

        println!("Upcoming spot: {}", checks[ind]);
        sleep(Duration::from_secs(2)).await;
        println!("Current spot: {}.", checks[ind]);
        sleep(Duration::from_secs(5)).await;
        ind += 1;
    }
}
