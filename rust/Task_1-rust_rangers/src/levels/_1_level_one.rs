use std::io;
use tokio::time::{sleep, Duration};

#[tokio::main]
pub async fn start_level_1() {
    println!("\n");
    let mut name: String = String::from("Rust_Rangers");
    println!("Hello, Welcome to the Forest {name}");
    println!("I can see you want to change the Forest name. Enter Y or y if so: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Please enter valid input"); // If `read_line` fails, the program will panic with this message

    match input.trim() {
        "Y" | "y" => {
            println!("Enter forest name: ");
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Please enter valid input");
            name = input.trim().to_string();
            println!("\n\n\nNew Forest name: {name}");
            println!("Great! As the variable storing Forest name is mutable, you were able to change the name. But be careful, not everything you want to change will be changable, because there are some items stored in variables which are immutable, you will not be able to change any of it.")
        }
        _ => {
            println!("Fine, the forest name will remain: {}", name);
        }
    }

    println!("\n\n\n");
    const AREA: u32 = 1;
    println!("Let us go ahead! We have a forest with a fixed size that cannot grow or shrink. The area of the forest is {AREA}sq. km. Can you guess which keyword is used to declare the area variable: let, let mut, or const?");

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Please enter valid input");

    match input.trim() {
        "let" => {
            println!("No, even though let creates an immutable variable, it does not guarantee that the value is fixed at compile time. Use let when the value is initialized once and doesn’t need to be constant. So use const when the value must be fixed and known at compile time.");
        }
        "let mut" => {
            println!("No, the forest area is fixed and should never change, but let mut allows you to modify the value. Therefore, it doesn't fit the requirement of an unchanging area. So use const when the value must be fixed and known at compile time.");
        }
        "const" => {
            println!("Correct. Since the forest area is fixed and known at compile time, const guarantees that it cannot be changed during the program’s execution.");
        }
        _ => {
            println!("No. Since the forest area is fixed and known at compile time, const guarantees that it cannot be changed during the program’s execution. So const is the correct answer.");
        }
    }

    
    let mut bamboop: f32 = 30.0;
    let mut ashokap: f32 = 20.0;
    let mut peepalp: f32 = 15.0;
    let mut banayanp: f32 = 15.0;
    let mut salp: f32 = 10.0;
    let mut otherp: f32 = 10.0;

    println!("\n\nIn the forest {bamboop}% of trees is Bamboo, {ashokap}% of trees is Ashoka tree, {peepalp}% of trees is Peepal, {banayanp}% of trees is Banyan, {salp}% of trees is Sal and {otherp}% of Others.");

    
    println!("You can increase or decrease the specific type of tree as per your wish. If someone wants to cut tree, you can grant permission.");

    let req: f32 = 5.5;
    let mut permission: bool = false;
    println!("Wait, someone wants {req}% of Bamboo trees out of {bamboop}%. Do you want to allow? Enter Y or y if so: ");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Please enter valid input"); // If `read_line` fails, the program will panic with this message

    match input.trim() {
        "Y" | "y" => {
            permission = true;
            println!("Permission granted");
        }
        _ => {
            println!("Permission denied");
        }
    }

    if permission {
        println!("Wait! Bamboo tress are being cut, It will take some time.");
        sleep(Duration::from_secs(5)).await;
        cut_tree(&mut bamboop, req);

        println!("What do you want to grow in that {req}%?, Bamboo? Ashoka? Sal? Peepal? Banayan? Other?");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Please enter valid input.");

        match input.trim().to_lowercase().as_str() {
            "bamboo" => {
                println!(
                    "Wait! {} tress are growing, It will take some time.",
                    input.trim()
                );
                sleep(Duration::from_secs(5)).await;
                plant_tree(&mut bamboop, req);
                println!("{}: {}", input.trim(), banayanp);
            }
            "ashoka" => {
                println!(
                    "Wait! {} tress are growing, It will take some time.",
                    input.trim()
                );
                sleep(Duration::from_secs(5)).await;
                plant_tree(&mut ashokap, req);
                println!("{}: {}", input.trim(), ashokap);
            }
            "sal" => {
                println!(
                    "Wait! {} tress are growing, It will take some time.",
                    input.trim()
                );
                sleep(Duration::from_secs(5)).await;
                plant_tree(&mut salp, req);
                println!("{}: {}", input.trim(), salp);
            }
            "peepal" => {
                println!(
                    "Wait! {} tress are growing, It will take some time.",
                    input.trim()
                );
                sleep(Duration::from_secs(5)).await;
                plant_tree(&mut peepalp, req);
                println!("{}: {}", input.trim(), peepalp);
            }
            "banayan" => {
                println!(
                    "Wait! {} tress are growing, It will take some time.",
                    input.trim()
                );
                sleep(Duration::from_secs(5)).await;
                plant_tree(&mut banayanp, req);
                println!("{}: {}", input.trim(), banayanp);
            }
            _ => {
                println!(
                    "Wait! {} tress are growing, It will take some time.",
                    input.trim()
                );
                sleep(Duration::from_secs(5)).await;
                plant_tree(&mut otherp, req);
                println!("{}: {}", input.trim(), otherp);
            }
        }
    }else {
        println!("Fine, you denied the permission to cut trees. The distribution of trees remain same.")
    }
    println!("\n");
    println!("Great! This level is complete. Let's move on to second level.");
}

fn cut_tree(treep: &mut f32, req: f32) {
    *treep = *treep - req;
}

fn plant_tree(treep: &mut f32, req: f32) {
    *treep = *treep + req;
}
