use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess number game begin");

    let target_number = rand::thread_rng().gen_range(1..101);
    loop {
        let mut user_number = String::new();

        io::stdin()
            .read_line(&mut user_number)
            .expect("read line err");
        
        // shadow user_number variant
        let user_number: u32 = match user_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match user_number.cmp(&target_number) {
            Ordering::Less => println!("Less than target number"),
            Ordering::Greater => println!("Greater than target number"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
