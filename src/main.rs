use std::io;
use std::cmp::Ordering;

fn main() {
    println!("1. C -> F");
    println!("2. F -> C");
    println!("3. Quit");

    let middle_choice = 2;

    loop {
        println!("Please input your choice.");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You choice: {}", choice);

        if choice > middle_choice {
            println!("Good evening!");
            break;
        }

        println!("Please input your degrees.");

        let mut degrees = String::new();

        io::stdin().read_line(&mut degrees)
            .expect("Failed to read line");

        let degrees: i32 = match degrees.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You choice: {}", degrees);

        match choice.cmp(&middle_choice) {
            Ordering::Less => println!("{} F", degrees*9/5 + 32),
            Ordering::Equal => println!("{} C", (degrees - 32)*5/9),
            _ => ()
        }
    }
}
