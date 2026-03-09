fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret: i32 = 42;

    let mut attempts = 0;

    let guesses = [10, 50, 30, 42];
    let mut index = 0;

    loop {
        let guess = guesses[index];
        attempts += 1;

        println!("Guess #{}: {}", attempts, guess);

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Correct! You guessed the secret number.");
            break;
        } else if result == 1 {
            println!("Too high!");
        } else {
            println!("Too low!");
        }

        index += 1;

        if index >= guesses.len() {
            println!("Ran out of guesses!");
            break;
        }
    }

    println!("It took {} guesses.", attempts);
}