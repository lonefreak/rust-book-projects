use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to 'Guess the number!' Type 'help' for the rules");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim() {
            "quit" | "q" => break println!("Quitting the game. Thanks for playing!"),
            "help" | "h" => {
                show_game_rules();
                continue;
            }
            _ => println!("You guessed {}", guess.trim()),
        };

        let guess: u32 = guess.trim().parse().expect("Please, type a number");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => break println!("You win!"),
        };
    }
}

fn show_game_rules() {
    println!(
        "
        ==== Guess the number ====

        When prompted, enter a number between 1 and 100.

        If the guessed number is higher than the correct number
        the text 'Too big!' will show and a new attempt
        will be prompted.

        If the gussed number is lower than the correct number
        the text 'Too small!' will show and a new attempt
        will be prompted.

        If the gussed number is correct the text 'You win!' will
        show and the game will end.

        You can write 'quit' in the prompt to stop the game.

        You can write 'help' in the prompt to read the game rules."
    );
}
