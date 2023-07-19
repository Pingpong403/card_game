use crate::card::card::Card;
use crate::card::card::Suits;
mod card;
use std::io;
use std::cmp::Ordering;

fn main() {
    // generate my random card
    let mut myCard: Card = Card { suit: Suits::Spades, value: 1 };
    myCard.generate();

    // let user guess my number
    println!("I have a card! Guess the number!");

    loop {
        let mut user_num_guess = String::new();
        io::stdin()
            .read_line(&mut user_num_guess)
            .expect("Failed to read line");
    
        let user_num_guess: u32 = match user_num_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {user_num_guess}");
    
        match user_num_guess.cmp(&myCard.value) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }

    println!("");

    println!("Now guess the suit!");
    println!("Hint: use 1 for Hearts, 2 for Diamonds, 3 for Spades, and 4 for Clubs");
    loop {
        let mut user_suit_guess = String::new();
        io::stdin()
            .read_line(&mut user_suit_guess)
            .expect("Failed to read line");

        let user_suit_guess: u32 = match user_suit_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {user_suit_guess}");

        if myCard.compare(&user_suit_guess) {
            println!("You got it!");
            break;
        }
        else {
            println!("Not quite!");
        }
    }

    println!("");
    
    println!("My card was:");
    myCard.display();
}
