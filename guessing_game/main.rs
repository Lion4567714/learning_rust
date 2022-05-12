use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    //  The "!" after println denotes the println macro is being called. Presumably that is what adds the newline.
    println!("Guess the number!");

    //  Variables are immutable by default. See line 16 for an example of a mutable variable.
    //  The ": u32" specifies the variable secret_number is an unsigned 32-bit integer.
    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        //  The .expect() function call is like an assert statement. If read_line() returns a Result as an error, 
        //      expect() will crash the program with the given error message.
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        //  match allows you to change what happens next based on the value of the given Result enum. Each statement
        //      is called an "arm".
        //  If parse() returns an Ok Result, the parsed value (num, contained within Ok) will be returned and assigned
        //      to guess with the "=>".
        //  If parse returns an Err Result, the returned error information (caught but the catchall value "_") is
        //      ignored and the next iteration of the loop begins.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>  {
                println!("You win!");
                break;
            }
        }
    }
}
