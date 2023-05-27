use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // println!("Guessing Game");
    // println!("Guess number : ");
    // let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("Secret number : {secret_number}");

    // let mut guess = String::new();

    // io::stdin()
    // .read_line(&mut guess)
    // .expect("faild to read guess number");

    // let guess:u32 = guess.trim().parse().expect("Type a number");

    // println!("your guess {}", guess);

    // match guess.cmp(&secret_number){
    //     Ordering::Less => println!("Your guess {} is less ",guess),
    //     Ordering::Greater => println!("Your guess {} is greater ",guess),
    //     Ordering::Equal => {
    //         print!("you win the game");

    //     }
    // }

    loop {
        println!("Guess a number:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Enter a number");
        let guess: u32 = guess.trim().parse().expect("Type a number");

        let secert_number = rand::thread_rng().gen_range(1..=10);

        match guess.cmp(&secert_number) {
            Ordering::Less => println!("Your guess {} is less ", guess),
            Ordering::Greater => println!("Your guess {} is greater ", guess),
            Ordering::Equal => {
                print!("you win the game");
                break;
            }
        }
    }
}
