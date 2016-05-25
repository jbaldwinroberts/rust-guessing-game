extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("");

    let secret = rand::thread_rng().gen_range(1, 101);

    loop {
	    let guess = read();
	    if guess == 0 {
	    	continue;
	    }
	    
	    let success = check(guess, secret);
	    if success {
	    	break;
	    }
	}
    
}

fn check(guess: u32, secret: u32) -> bool {
    match guess.cmp(&secret) {
		Ordering::Less		=> {
			println!("Too small!");
			false
		}
		Ordering::Greater	=> {
			println!("Too big!");
			false
		}
		Ordering::Equal		=> {
			println!("You win!");
			true
		}
	}
}

fn read() -> u32 {
	println!("Input your guess:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read guess!");
    let guess: u32 = match guess.trim().parse() {
    	Ok(num)	=> num,
    	Err(_)	=> {
    		println!("Please enter a number!");
    		0
    	}
    };
    guess
}
