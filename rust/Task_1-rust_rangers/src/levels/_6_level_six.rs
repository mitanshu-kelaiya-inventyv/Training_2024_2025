use std::{io};
use tokio::time::{sleep, Duration};
pub trait Explorer {
    fn introduce(&self);
    fn get_name(&self) -> String;
    fn get_group_name(&self) -> String;
}

#[derive(Debug)]
enum SkillLevel {
    BEGINEER,
    INTERMEDIATE,
    EXPERT,
}

#[derive(Debug)]
struct Guide {
    name: String,
    caves_explored: u8,
    country: String,
    languages: [String; 3],
    specialities: [String; 3],
    fee_per_day: i32,
    group_name: String,
    skill_level: SkillLevel,
}

#[derive(Debug)]
struct Traveler {
    name: String,
    caves_explored: u8,
    country: String,
    languages: [String; 3],
    skill_level: SkillLevel,
    group_name: String,
}

impl Guide {
    fn compare(&self, against: &Guide) {
        println!("\t\t\t{:20}\t\t\t{}", self.name, against.name); //Formatted Printing
        println!(
            "Languages Known: {:35?} \t {:71?}",
            self.languages, against.languages
        );
        println!(
            "Caves explored : {:35}\t{:71}",
            self.caves_explored, against.caves_explored
        );
        println!(
            "Skill Level    : {:35?} \t {:71?}",
            self.skill_level, against.skill_level
        );
        println!(
            "Specialities   : {:?} \t {:?}",
            self.specialities, against.specialities
        );
        println!(
            "Fees per Day   : {:35}   \t {:71}",
            self.fee_per_day, against.fee_per_day
        );
    }

    fn climbing_tips(&self){
        if self.specialities.contains(&String::from("Climbing and Rappelling")){
            println!("As a climbing and rappelling specialist, I advise always maintaining three points of contact for better stability. Focus on using your legs to bear your weight, as they are stronger than your arms. Double-check your harness, knots, and anchor points before each climb or rappel. Plan your route in advance and identify rest spots to avoid fatigue. Remember, controlled and steady movements are key to success.");
            println!("Guide is demonstrating on how to climb.");
        }else{
            println!("I recommend keeping your body close to the rock for better balance and stability. Always ensure your harness is secure and wear a helmet to protect against falling debris. Take your time and avoid rushing; safety comes first. Stick to beginner-friendly routes and follow your team leader. And don't forget to hydrate and take breaks when needed!");

        }
    }

    fn wildlife_info(&self){
        if self.specialities.contains(&String::from("Wildlife Identification")){
        println!("As a wildlife specialist, I can identify this species as the 'Cave Bat' (Myotis velifer), which plays a crucial role in maintaining the cave's ecosystem. These bats are insectivores, helping control populations of cave-dwelling insects. The guano left behind by these bats is also an important nutrient source for other organisms. Their presence indicates a healthy, balanced cave ecosystem, as they thrive in caves with stable conditions and minimal human disturbance.");
        }else{
            println!("This looks like a bat, possibly a cave bat, but I’m not entirely sure of the species. Bats like these are common in caves and are known to feed on insects. They are beneficial for controlling insect populations, which is important for the cave’s environment. But, I can't go into much detail beyond that. We should keep a safe distance from them, as they can be sensitive to human presence.");

        }

    }

    fn cave_history(&self){
        if self.specialities.contains(&String::from("Cave Science")) {
            println!("These carvings are part of an ancient tradition likely dating back thousands of years. Based on the composition of the stone and the specific tool marks, it's clear that these were created using primitive tools, possibly flint or bone. The carvings themselves likely depict ceremonial scenes, daily life, or mythological events, which is common in the art left by early human cultures. In caves like these, we often find that the artwork is associated with rituals, and it could have had spiritual significance for those who created it. The fact that these carvings have been preserved so well speaks to the stable environment of the cave, which has protected them from weathering and damage. It's important to approach these with respect, as they represent a connection to a culture that once lived here, and we must work to preserve them for future generations of researchers.");
        } else {
            println!("These sculptures and carvings are fascinating! They are really old, possibly made by early humans who lived here. WThis carvings depict things like daily life or ceremonies.The repetitive patterns and symbols suggest that these might have had some cultural or ceremonial significance.");
        }
    }

    fn cave_navigation(&self){
        if self.specialities.contains(&String::from("Cave Navigation")){
            println!("As a cave navigation specialist, I recommend using a methodical approach when navigating underground streams. Always stay close to the wall, as the current can be deceptive and strong. Mark your route with biodegradable markers to ensure you don't lose your way. Before entering the stream, check the water level and flow speed to assess the risks. And always ensure that your group moves together to avoid separation in case of sudden changes in the water flow.");

        }else{
            println!("When navigating an underground stream, I suggest staying alert for any changes in the water's flow. Make sure to move slowly and watch for slippery rocks, as they can easily cause falls. It’s helpful to stick to the edges of the stream where possible, but be cautious of submerged rocks. Keep an eye on your group and ensure everyone stays close, especially if the water becomes deeper or faster.");
        }
    }
}

impl Traveler{
    fn take_note(&self, note:String){
        
    }
}

impl Explorer for Guide {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_group_name(&self) -> String {
        self.group_name.clone()
    }
    fn introduce(&self) {
        println!("Hello, My name is {}.\nI have explored {} caves.\nI belong to {}.\nI know languages {:?}.\nI am {:?} in cave exploration.\nMy per day charge is {}.", self.name, self.caves_explored, self.country, self.languages, self.specialities,self.fee_per_day);
    }
}

impl Explorer for Traveler {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn introduce(&self) {
        println!("Hello, My name is {}, Nice to meet you.\nI have explored {} caves.\nI belong to {}.\nI know languages {:?}.\nMy skill level is: {:?}",self.name, self.caves_explored,self.country, self.languages, self.skill_level);
    }

    fn get_group_name(&self) -> String {
        self.group_name.clone()
    }
}




#[tokio::main]
pub async fn start_level_6() {
    println!("LEVEL 6");
    let mut input = String::new();
    let guide_1 = Guide {
        name: String::from("Ramesh"),
        caves_explored: 12,
        country: String::from("India"),
        languages: [
            String::from("English"),
            String::from("Hindi"),
            String::from("Marathi"),
        ],
        specialities: [
            String::from("Climbing and Rappelling"),
            String::from("Wildlife Identification"),
            String::from("Cave Science"),
        ],
        fee_per_day: 1000,
        group_name: String::from("Group Blackstone"),
        skill_level: SkillLevel::INTERMEDIATE,
    };

    let guide_2 = Guide {
        name: String::from("Suresh"),
        caves_explored: 36,
        country: String::from("India"),
        languages: [
            String::from("English"),
            String::from("Hindi"),
            String::from("Spanish"),
        ],
        specialities: [
            String::from("Weather and Environmental Knowledge"),
            String::from("Emergency First Aid"),
            String::from("Cave Navigation"),
        ],
        fee_per_day: 1200,
        group_name: String::from("Group Nightshade"),
        skill_level: SkillLevel::EXPERT,
    };

    println!("We are going to explore cave. You have 2 options for choosing a guide. Let me introduce guides to you.");
    guide_1.introduce();
    println!("\n\n");
    guide_2.introduce();
    let my_guide;
    //guide_1.compare(&guide_2);

    loop {
        println!("Which guide do you want to choose? Press key 1 for guide {} and key 2 for guide {}.\nIf you want to compare both guides, press key 3.", guide_1.get_name(), guide_2.get_name());
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Some error occured");
        match input.trim() {
            "1" => {
                my_guide = &guide_1;
                println!(
                    "{} is your guide and your group name is {}.",
                    my_guide.get_name(),
                    my_guide.get_group_name()
                );
                break;
            }
            "2" => {
                my_guide = &guide_2;
                println!(
                    "{} is your guide and your group name is {}.",
                    my_guide.get_name(),
                    my_guide.get_group_name()
                );
                break;
            }
            "3" => {
                guide_1.compare(&guide_2);
            }
            _ => {
                println!("Please enter valid key.");
            }
        }
    }

    let me = Traveler {
        name: String::from("Akshay"),
        caves_explored: 0,
        country: String::from("India"),
        languages: [
            String::from("English"),
            String::from("Hindi"),
            String::from("Maithili"),
        ],
        skill_level: SkillLevel::BEGINEER,
        group_name: my_guide.get_group_name(),
    };

    println!("Let's introduce yourself to your guide.");
    me.introduce();

    println!("Let's start exploring cave.");

    let cave_checkpoints = [
        String::from("Narrow Passage"),
        String::from("Underground Stream"),
        String::from("Wildlife Encounter"),
        String::from("Sculptures and Carvings")
    ];

    for location in &cave_checkpoints{
        println!("\n\n{} ahead.\n", {location});
        sleep(Duration::from_secs(2)).await;
        match location.as_str(){
            "Narrow Passage" =>{
                println!("A narrow and steep section of the cave requires rappelling to move forward.");
                println!("Guide giving instruction: ");
                my_guide.climbing_tips();
                sleep(Duration::from_secs(5)).await;
            },
            "Underground Stream"=>{
                println!("An underground stream that diverts the path");
                println!("Guide giving instruction: ");
                my_guide.cave_navigation();
                sleep(Duration::from_secs(5)).await;
            },
            "Wildlife Encounter"=>{
                println!("We can spot bats, insects, or rare cave-dwelling creatures. The guide will explains their ecological significance.");
                println!("Guide giving information: ");
                my_guide.wildlife_info();
                sleep(Duration::from_secs(5)).await;
            },
            "Sculptures and Carvings"=>{
                println!("There are sculptures and carving here.");
                println!("Guide giving information: ");
                my_guide.cave_history();
                me.take_note(String::new());
                sleep(Duration::from_secs(5)).await;
            },
            _=>{
                println!("Moving Ahead........");
            }
        }
    }

    println!("\n\n\nHere, we have two entities Guide and Travelere. Since Guide and Traveler both are Explorer, they share common behaviour like name, introducing each other and they have group name. If someone is Guide or Traveler they must have this callable behaviours. So having a trait is suitable for this condition.\n\n\nGreat! we are trough to the cave. See you again at next level.")
}
