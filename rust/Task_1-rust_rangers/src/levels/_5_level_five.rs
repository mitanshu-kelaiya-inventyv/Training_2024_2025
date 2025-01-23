use std::io;
use tokio::time::{sleep, Duration};

// - Level 5 – The Fields of Structs and Enums
// 	- User Story:
// 		- As a player, I want to define and use structs and enums in Rust to represent complex data types. I should complete tasks that require creating custom data structures to better manage and manipulate game entities.
// 	- Question: How do you define and use structs and enums in Rust to organize complex data?
#[derive(Debug)]
enum Item {
    GOLD,
    SILVER,
    BRONZE,
    COAL,
    DIAMOND,
    EMPTY,
}

#[derive(Debug)]
struct Player {
    name: String,
    treasure: [(Item, i32); 5],
    money: i32,
}

impl Player {
    async fn sell(&mut self, mut input: &mut String) {
        println!("What to sell?, \n0: for COAL, \n1: for BRONZE, \n2: for SILVER, \n3: for GOLD, \n4: for DIAMOND \nany other key will return you back.");
        input.clear();
        io::stdin().read_line(&mut input).expect("Invalid input");
        let to_sell = input.trim().parse().expect("Incorrect digit");
        println!("Selling your items........");
        sleep(Duration::from_secs(5)).await;
        
        match to_sell {
            0 => {
                let val = self.treasure[0].1;
                self.treasure[0] = (Item::COAL, 0);
                self.money = self.money + val;
            },
            1 => {
                let val = self.treasure[1].1;
                self.treasure[1] = (Item::BRONZE, 0);
                self.money = self.money + (val*3);
            },
            2 => {
                let val = self.treasure[2].1;
                self.treasure[2] = (Item::SILVER, 0);
                self.money = self.money + (val*5);
            },
            3 => {
                let val = self.treasure[3].1;
                self.treasure[3] = (Item::GOLD, 0);
                self.money = self.money + (val*10);
            },
            4 => {
                let val = self.treasure[4].1;
                self.treasure[4] = (Item::DIAMOND, 0);
                self.money = self.money + (val*20);
            },
            _ => {
                println!("Returning back...");
            }
        }
    }
}

#[tokio::main]
pub async  fn start_level_5() {
    let treasure: [(Item, i32); 5] = [
        (Item::COAL, 0),
        (Item::BRONZE, 0),
        (Item::SILVER, 0),
        (Item::GOLD, 0),
        (Item::DIAMOND, 0),
    ];
    


    let mut player_new = Player {
        name: String::from("JitteryGold"),
        treasure,
        money: 0,
    };


    println!("Level 5!!, Welcome to the gold field. You will be a miner today with the name : {}.", player_new.name);

    let mut mine: [[Item; 5]; 5] = get_a_mine();
    let mut input: String = String::from("");

    println!("You are having a field 5x5 and you are having appropriate tools to mine. You can move up, down , left and right in the mine but cant directly jump to any position.\nPress key U for moving up, \nD for moving down, \nL for moving left and \nR for moving right. \nAny other key will take you out of mine");

    mining(0, 0, &mut input, &mut mine, &mut player_new);

    println!("{:?}", player_new.treasure);

    player_new.sell(&mut input).await;

    println!("Final Status: ");
    println!("{:?}", player_new);

    println!("\n\nYour profile is created using structure type. Structure type can group items of a single entity with heterogeneous types. A player profile do have 3 fields: name, current items in treasure and money. Treasure is represented by enums because items in treasure are fixed, either it is either a Coal, a Silver, a Gold or a Diamond. So treasure does store enum variants. So this is how struct and enum is managing complex profile type and fixed items in enum");
    println!("\n\nCongratulations,Now you have your experience in Structures and Enums in RUST. See you at level 6.")
}




fn get_a_mine() -> [[Item; 5]; 5] {
    let mine: [[Item; 5]; 5] = [
        [
            Item::EMPTY,
            Item::COAL,
            Item::EMPTY,
            Item::BRONZE,
            Item::EMPTY,
        ],
        [
            Item::GOLD,
            Item::COAL,
            Item::EMPTY,
            Item::EMPTY,
            Item::EMPTY,
        ],
        [
            Item::EMPTY,
            Item::EMPTY,
            Item::SILVER,
            Item::EMPTY,
            Item::GOLD,
        ],
        [
            Item::COAL,
            Item::EMPTY,
            Item::EMPTY,
            Item::DIAMOND,
            Item::EMPTY,
        ],
        [
            Item::COAL,
            Item::EMPTY,
            Item::EMPTY,
            Item::EMPTY,
            Item::DIAMOND,
        ],
    ];

    mine
}



fn mining(
    row: usize,
    col: usize,
    mut input: &mut String,
    mut mine: &mut [[Item; 5]; 5],
    player_new: &mut Player,
) {

    println!("\n\n");
    for i in 0..5 {
        for j in 0..5 {
            print!("{:?} ", mine[i][j]);
        }
        println!();
    }

    println!("Reached location: {row} {col}");

    match mine[row][col] {
        Item::COAL => {
            let val = player_new.treasure[0].1;
            player_new.treasure[0] = (Item::COAL, val + 1);
            mine[row][col] = Item::EMPTY;
        }
        Item::BRONZE => {
            let val = player_new.treasure[1].1;
            player_new.treasure[1] = (Item::BRONZE, val + 1);
            mine[row][col] = Item::EMPTY;
        }
        Item::SILVER => {
            let val = player_new.treasure[2].1;
            player_new.treasure[2] = (Item::SILVER, val + 1);
            mine[row][col] = Item::EMPTY;
        }
        Item::GOLD => {
            let val = player_new.treasure[3].1;
            player_new.treasure[3] = (Item::GOLD, val + 1);
            mine[row][col] = Item::EMPTY;
        }
        Item::DIAMOND => {
            let val = player_new.treasure[4].1;
            player_new.treasure[4] = (Item::DIAMOND, val + 1);
            mine[row][col] = Item::EMPTY;
        }
        Item::EMPTY => {}
    }

    input.clear();
    io::stdin().read_line(&mut input).expect("Invalid input");
    println!("{}", input.trim().to_uppercase().as_str());

    match input.trim().to_uppercase().as_str() {
        "U" => {
            if row == 0 {
                println!("Cannot move up");
                mining(row, col, input, &mut mine, player_new);
                return;
            }
            println!("Going location: {} {col}", row-1);
            mining(row - 1, col, input, &mut mine, player_new);
        }
        "D" => {
            if row == 4 {
                println!("Cannot move down");
                mining(row, col, input, &mut mine, player_new);
                return;
            }
            println!("Going location: {} {col}", row+1);
            mining(row + 1, col, input, &mut mine, player_new);
        }
        "L" => {
            if col == 0 {
                println!("Cannot move left");
                mining(row, col, input, &mut mine, player_new);
                return;
            }
            println!("Going location: {row} {}", col-1);
            mining(row, col - 1, input, &mut mine, player_new);
        }
        "R" => {
            if col == 4 {
                println!("Cannot move right");
                mining(row, col, input, &mut mine, player_new);
                return;
            }
            println!("Going location: {row} {}",col+1);
            mining(row, col + 1, input, &mut mine, player_new);
        }
        _ => {
            println!("Exiting Mine");
            return;
        }
    }
}
