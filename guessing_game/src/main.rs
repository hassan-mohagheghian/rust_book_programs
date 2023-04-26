use rand;
use std::io;
fn main() {
    let random_number = rand::random::<u8>();
    println!("random is {}:", random_number);
    println!("--------- Guess the number! ---------");
    println!("-- The guess number is in (0..255) --");

    loop {
        let mut input = String::new();
        println!("-- please enter your guess: ");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(err) => {
                println!("-- In reading your guess an error occured: {}", err);
                continue;
            }
        }

        println!("{}", input);

        let guess_number = match input.trim().parse::<u8>() {
            Ok(num) => num,
            Err(err) => {
                println!(
                    "-- Your guess is not a number in (0..255). please retry. {}",
                    err
                );
                continue;
            }
        };

        if random_number == guess_number {
            println!("---- Hooray!. Your guess is correct. You won ----");
            break;
        } else if random_number < guess_number {
            println!("-- Your guess is big. please select lesser than.");
        } else {
            println!("-- Your guess is small. please select greater than.");
        }
    }
}
