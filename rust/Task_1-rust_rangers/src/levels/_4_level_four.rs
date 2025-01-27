use tokio::time::{sleep, Duration};
use rand::Rng;
use std::io;

#[tokio::main]
pub async fn start_level_4() {
    println!("Level Four");
    println!("You are in a dark mine. You have been provided with a private key. Enter the key exactly as it is. You will then travel in a trolley to reach the field. Be careful while driving and ensure you drive safely.");

    let private_key = "ABC123";
    let mut count: u8 = 0;
    let mut input = String::from("");
    loop {
        count += 1;
        println!("Enter your private key: ");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Please enter valid input!!");

        if validate_key(input.trim(), &private_key) {
            println!("Wait a sec, doors are opening....");
            break;
        } else {
            println!(
                "You have tried {count} times, you have {} attempts left.",
                3 - count
            );
        }

        if count == 3 {
            panic!("You have tried maximum number of times you are out now.");
        }
    }

    let roads: [&str; 4] = [
        "Rockslide Alley",
        "Collapsed Shaft",
        "Dusty Passage",
        "Vein Rush",
    ];
    println!("\n\n\nHere is your trolley. You can set the speed of the trolley as you wish. The trolley will provide information about the road ahead. Adjust your speed according to the road type. Overspeeding may lead to dangerous situations, as accidents cannot be reversed.");
    for i in 0..7 {
        let road_type_index = rand::thread_rng().gen_range(0..=3);
        println!("\n\nRoad Type: {}", roads[road_type_index]);
        input.clear();
        match roads[road_type_index] {
            "Rockslide Alley" => {
                println!("Speed Range suggested: 1-10");
                let speed:u64 = take_speed(&mut input);

                let panic: bool = check_panic(10, speed);

                if panic {
                    panic!("Victim of accident. WASTED. Speed limit exceeded.");
                } else {
                    let sub = (speed as f64 * 0.1) as u64;
                    println!("Going at speed {}", speed);
                    sleep(Duration::from_secs(12 - sub)).await;
                }
            }
            "Collapsed Shaft" => {
                println!("Speed Range suggested: 0-30");
                let speed = take_speed(&mut input);

                let panic: bool = check_panic(30, speed);

                if panic {
                    panic!("Victim of accident. WASTED. Speed limit exceeded.");
                } else {
                    let sub = (speed as f64 * 0.1) as u64;
                    println!("Going at speed {}", speed);
                    sleep(Duration::from_secs(12 - sub)).await;
                }
            }
            "Dusty Passage" => {
                println!("Speed Range suggested: 0-50");
                let speed = take_speed(&mut input);

                let panic: bool = check_panic(50, speed);

                if panic {
                    panic!("Victim of accident. WASTED. Speed limit exceeded.");
                } else {
                    
                    let sub = (speed as f64 * 0.1) as u64;
                    println!("Going at speed {}", speed);
                    sleep(Duration::from_secs(12 - sub)).await;
                }
            }
            "Vein Rush" => {
                println!(
                    "Suggested Speed Range: 0-120. While the trolley can exceed 80, it may lead to instability and accidents. It is strongly advised to keep your speed at or below 80 for safety."
                );                
                let speed = take_speed(&mut input);

                let panic: bool = check_panic(120, speed) | may_panic(speed);
                if panic {
                    panic!("Victim of accident. WASTED. Speed limit exceeded.");
                } else {
                    let sub = (speed as f64 * 0.1) as u64;
                    println!("Going at speed {}", speed);
                    sleep(Duration::from_secs(12 - sub)).await;
                }
            }
            _ => {}
        }
    }

    println!("Reached the Field. Now on to the LEVEL 5. See you later.....")
}

fn validate_key(input: &str, key: &str) -> bool {
    return input == key;
}

fn take_speed(mut input: &mut String) -> u64 {
    let mut speed = 0;
    while speed == 0 {
        input.clear();
        println!("Enter your speed: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Please enter valid input!!");
        speed = match input.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Recoverable error detected. You have entered invalid input. Please enter correct input.");
                0
            }
        }
    }

    return speed;
}

fn check_panic(max_speed: u64, user_speed: u64) -> bool {
    user_speed > max_speed
}

fn may_panic(user_speed: u64) -> bool {
    if user_speed <= 80 {
        false
    } else {
        if rand::thread_rng().gen_range(0..=1) == 0 {
            false
        } else {
            true
        }
    }
}
