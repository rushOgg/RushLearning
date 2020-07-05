use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Please input your num.");
    
    
    let num = rand::thread_rng().gen_range(1,100);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Wrong Num");
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess.cmp(&num) {
            Ordering::Equal => {
                println!("Equal");
                break;
            },
            Ordering::Greater => println!("Too bigger"),
            Ordering::Less => println!("Too Small"),
        }
    }

    
    println!("guess num is {}.", num);
}