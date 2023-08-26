use std::io;
use crook_calculator::compute;

fn main() {
    println!("Welcome to Level 1 Crook Calculator.");

    'main_loop:
    loop {
        println!("Enter new expression:");
        
        let mut input = String::new();
        if let Err(_) = io::stdin().read_line(&mut input) {
            continue 'main_loop;
        }

        if let Some(res) = compute(&input) {
            println!("Result: {}", res);
        }
    }
}
