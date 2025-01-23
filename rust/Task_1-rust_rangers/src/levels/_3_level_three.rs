use std:: io;
use tokio::time::{sleep, Duration};

#[tokio::main]
pub async fn start_level_3(){
    println!("Welcome to level three. You are at the bottom of 'THE RUSTY MOUNTAIN'.");
    sleep(Duration::from_secs(2)).await;
    println!("\n\nWe will learn ownership and borrowing concepts trekking through level 3. We will have situations where we will take ownership, pass mutable and immutable references.");
    sleep(Duration::from_secs(2)).await;
    println!("\n\n\nYou need to trekk the mountain to move to the next level. There are some people here, together with you. You have a bag of items. The items include Food and Water, Cameras, Emergency kit, Camping gears, Head lamps.");
    println!("So let's start our journey. First of all we will require a copy of map. Let us have two copies of map.");

    let locations: [(String, String); 5] = [
        (String::from("Trailhead"), String::from("North.")),
        (String::from("Waterfall Lookout"), String::from("East.")),
        (String::from("Forest Clearing"), String::from("Straight.")),
        (String::from("Rocky Ridge"), String::from("West.")),
        (String::from("Mountain Cabin"), String::from("Southwest.")),
    ];

    let player_map_1: [(String, String); 5] = locations.clone();
    let mut photos:[String;5]=core::array::from_fn(|_| String::from(""));
    let mut food: [(&str, i32); 3] =[("Banana", 20), ("Apples", 15), ("Carrot", 15)];

    for (location, direction) in player_map_1.iter(){
        println!("\n\nGoing location {location} in {direction}.");
        sleep(Duration::from_secs(3)).await;
        println!("Reached loactaion {location}.");
        println!("\n\n");
        match location.as_str(){
            "Waterfall Lookout" =>{
            println!("Let's have a photo session. The photographer will take some photos of you. He will pass on the photos. Earlier, the photographer was the owner of the photos since they were with him, but now he will pass ownership to us and he won't be able to access the photos anymore.");
            sleep(Duration::from_secs(5)).await;
            photos = photo_session();
            
        },
        "Forest Clearing"=>{
            println!("Alice is asking if you can give him the map because he has lost his. Alice wants to take a photo of the map. Alice wants to borrow the map, so let’s help him. Since we don’t want Alice to change anything inside the map, we will pass it as an immutable reference.");
            pass_the_map(&player_map_1);
            sleep(Duration::from_secs(5)).await;
        },
        "Rocky Ridge"=>{
            food[0].1=2;
            food[1].1=3;
            food[2].1=1;
            println!("There's a fruit and veggie shop here. Since we are almost out of fruit and veggies, let’s refill our stock. We will pass our food bag to refill it, but only to avoid carrying extra bags. So here, we are passing a mutable reference of our food bags.");
            pass_food_bag(&mut food);
            println!("Waiting for fill....");
            sleep(Duration::from_secs(5)).await;
            for (name, count) in food.iter(){
                println!("{name} {count}");
            }
        },
        "Mountain Cabin"=>{
            println!("Look who it is! Bob your friend is here. He is asking for the photos you took at the waterfall. Let’s pass those photos by using the borrowing concept in Rust.");
            pass_photos(&photos);
            sleep(Duration::from_secs(5)).await;
        },
        _=>{
            println!("Moving Ahead....");
        }
        }
    }

    println!("Good job, mate! It was a long journey, but now let’s head to the other side to reach level 4.");

    sleep(Duration::from_secs(10)).await;

    println!("Wohoo! we are back to the ground and through to the level 3.")


}

//Passing the ownership
fn photo_session()->[String;5]{
    
   let mut ind:usize=0;
    println!("5 Photos will be clicked");
    let photos = [String::from("Waterfall Wonder"), String::from("Mist in Motion"), String::from("Nature's Beauty"), String::from("Cascade Moment"), String::from("Flowing Serenity")];
    
    while ind<5{
        println!("Photo {} clicked.", photos[ind]);
        ind+=1;
    }
    println!("Photographer is passing the ownership.....");
    photos
       
}

//Immutable Borrowing
fn pass_the_map(player_map_2:&[(String, String);5]){
    println!("Alice: Hey mate, Thanks for the map.");
}

//Immutable Borrowing
fn pass_photos(photos:&[String;5]){
    println!("Bob: Nice");
}

//Mutable Borrowing
fn pass_food_bag(food:&mut [(&str, i32); 3]){
    let mut guess=String::from("");
    for i in 0..food.len(){
        println!("How many {}?", food[i].0);
        guess.clear();
        io::stdin().read_line(&mut guess).expect("Invalid input");
        let cnt:i32 = match guess.trim().parse(){
            Ok(num)=>{
                num
            },
            Err(err)=>{
                println!("{err}");
                food[i].1
            }
        };
        food[i].1=cnt;
    }
    
}


