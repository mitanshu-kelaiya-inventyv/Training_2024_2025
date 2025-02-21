//Mutex
//RwLock
//Rc&Arc

use std::{sync::{Arc, RwLock}, thread::{self, sleep}, time::Duration};
use rand::Rng;

#[derive(Debug)]
struct User{
    id: String,
    name:String
}

fn id_generator() -> String {
    let mut rng = rand::thread_rng();
    let id: String = (0..10)
        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
        .collect();
    
    id
}
fn main() {
    let users: Arc<RwLock<Vec<User>>> = Arc::new(RwLock::new(vec![]));
    let add_user_clone = users.clone();
    let print_len_user_clone = users.clone();
    let remove_user_clone = users.clone();
    let print_user_clone = users.clone();
    let let_panic_clone = users.clone();


    let add_user = thread::spawn(move || {
        loop {
            let user = User{
                id:id_generator(),
                name:String::from("Mohit")
            };
            add_user_clone.write().unwrap().push(user);
            thread::sleep(Duration::from_millis(500));
        }       
});



    let print_len = thread::spawn(move||{
        loop{
            println!("Length of users vector: {}", print_len_user_clone.read().unwrap().len());
            thread::sleep(Duration::from_secs(2));
        }
    });

    let remove_user = thread::spawn(move||{
        loop{
            if remove_user_clone.read().unwrap().len() > 0{
            remove_user_clone.write().unwrap().swap_remove(0);
        }
            sleep(Duration::from_secs(4));
        }
    });

    let print_user = thread::spawn(move ||{
        loop{
            println!("{:?}", print_user_clone.read());
            sleep(Duration::from_secs(3));
        }
    });

    let let_panic = thread::spawn(move ||{
        loop{

            if let_panic_clone.read().unwrap().len() > 100{
        // .join() only returns Err(_) if the thread panicked! 
        // It does not return Err(...) for other errors inside the thread.
                panic!();
            }else{
                sleep(Duration::from_millis(100));
            }
            
            
        }
    });



    match let_panic.join() {
        Ok(_) => println!("Thread exited successfully."),
        Err(err) => panic!("Thread panicked, Length exceeded 100"),
    }

    
}
