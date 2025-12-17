use std::{io::{Write, stdin, stdout}, thread::sleep, time::Duration};

fn main() {
    let password = String::from("SECRET");

    loop {
        let mut input:String = String::new();

        print!("Enter your password: ");
        stdout().flush().expect("Failed to flush stdout!");

        stdin()
            .read_line(&mut input)
            .expect("Read error!");

        let input:&str = input.trim();

        if password == input {
            println!("Correct password!");
            sleep(Duration::from_secs(20));
            break;
        } else {
            println!("Password incorrect!");
        }
    }
}
