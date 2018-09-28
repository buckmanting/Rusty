extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let game: (u32, u32) = setup_game();
    let answer: u32 = game.0;
    let _number_of_players: u32 = game.1;

    loop {
        println!("\nPlayer {}, please input your guess...", 1);

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&answer) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn setup_game() -> (u32, u32) {
    let mut _number_of_players: String = String::from("");

    println!("Lets play guess the number!");

    let answer_limit = get_difficulty();

    let answer = rand::thread_rng().gen_range(1, answer_limit);

    let _number_of_players: u32 = get_number_of_players(&mut _number_of_players);

    (answer, _number_of_players)
}

fn get_difficulty() -> u32 {
    let mut difficulty_level: String = String::new();
    let answer_limit: u32;

    println!("What difficulty would you like? Easy (1-100), Medium (1-500), Hard(1-10,000)");

    loop {
        io::stdin().read_line(&mut difficulty_level)
            .expect("Failed to read line");

        let difficulty_level = String::from(difficulty_level.trim());
        match &difficulty_level {
            _ if difficulty_level == "easy" => {
                answer_limit = 101;
                break;
            }
            _ if difficulty_level == "medium" => {
                answer_limit = 501;
                break;
            }
            _ if difficulty_level == "hard" => {
                answer_limit = 10_001;
                break;
            }
            _ => {
                println!("{} is not a valid difficulty level, please enter a valid difficulty", difficulty_level.trim())
            }
        }
    }
    answer_limit
}

fn get_number_of_players(mut _number_of_players: &mut String) -> u32 {
    println!("How many people are playing (1-10)?");
    io::stdin().read_line(&mut _number_of_players)
        .expect("Failed to read line");
    let _number_of_players: u32 = _number_of_players.trim().parse().unwrap();
    _number_of_players
}
