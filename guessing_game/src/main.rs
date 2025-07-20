use rand::{Rng, rng};
use std::{cmp::Ordering, io};

struct GuessData {
    secret: u32,
    number_of_attempts: u32,
}

impl GuessData {
    fn new() -> GuessData {
        GuessData {
            secret: get_random(),
            number_of_attempts: 0,
        }
    }

    fn reset(&mut self) {
        self.secret = get_random();
        self.number_of_attempts = 0;
    }

    fn increment_attempt_counter(&mut self) {
        self.number_of_attempts += 1;
    }
}

fn get_random() -> u32 {
    rng().random_range(1..=100)
}

fn read_user_guess() -> Result<u32, String> {
    let mut guess_str = String::new();
    io::stdin()
        .read_line(&mut guess_str)
        .map_err(|e| e.to_string())?;

    guess_str.trim().parse::<u32>().map_err(|e| e.to_string())
}

fn read_user_input_on_game_state() -> Result<bool, String> {
    println!("Do you want to keep playing?");
    println!("Type CONTINUE to start a new game, anything else to quit.");
    let mut input_str = String::new();
    io::stdin()
        .read_line(&mut input_str)
        .map_err(|e| e.to_string())?;
    match input_str.trim() {
        "CONTINUE" => Ok(true),
        _ => Ok(false),
    }
}

fn main() {
    let mut guess_data = GuessData::new();
    loop {
        println!("Guess a number between 1 and 100 (inclusive)!");
        let guess = read_user_guess();
        guess_data.increment_attempt_counter();
        let guess = match guess {
            Ok(num) => num,
            Err(msg) => {
                println!("Could not parse number: {}", msg);
                continue;
            }
        };
        match guess.cmp(&guess_data.secret) {
            Ordering::Greater => {
                println!("Your guess {} is too large.", guess);
                continue;
            }
            Ordering::Less => {
                println!("Your guess {} is too small.", guess);
                continue;
            }
            Ordering::Equal => {
                println!("Congratulations, your guess {} was correct!", guess);
                println!("It took you {} attempts.", guess_data.number_of_attempts)
            }
        };
        match read_user_input_on_game_state() {
            Ok(verdict) => {
                if verdict {
                    guess_data.reset();
                    println!("Starting new game.");
                    continue;
                } else {
                    println!("Quitting.");
                    break;
                }
            }
            Err(msg) => {
                println!("Could not process user input: {}", msg);
                println!("Quitting game due to error.");
                break;
            }
        }
    }
}
