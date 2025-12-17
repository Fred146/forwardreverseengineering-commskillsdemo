# Forward / Reverse Engineering Demo

<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/afc67647-b6f8-475f-a682-c41f9e9b1e92" />
[(Image courtesy of Triple Infotech)](https://tripleinfotech.com/2024/08/29/reverse-vs-forward-engineering-key-differences-and-when-to-choose)

## 1. Contact
Feel free to open a ticket if you experience any difficulties following the instructions below.
<br><br>

## 2. Forward Engineering
In this demo, we use **Rust** as the programming language and **Visual Studio Code** as the source code editor. You are free to use any platform or tools of your choice, as the outcome will be very similar. You may also skip this chapter entirely if you are only interested in the reverse-engineering part; an executable binary will be provided.
<br><br>

### 2.1 Project Initiation

Initiate a new **Rust** project using the following command in **Windows**, in a directory of your choice:
```
cargo new commskillsdemo
```
<br>

Afterwards, open the following file in your source code editor:
```
src\main.rs
```
<br>

<img width="979" height="225" alt="image" src="https://github.com/user-attachments/assets/d34b5f76-5c41-4b07-b7ee-092bec20a3f1" />
<br><br>

### 2.2 Source Code

You will be presented with the following code:
<br><br>

```rust
fn main() {
    println!("Hello, world!");
}
```

<br>
You may remove all of these lines of code and replace them with the code below. Each section of the code will be explained in more detail afterward.
<br><br>

```rust
// ** Warning **  
// This code is intentionally insecure and is meant for educational and
// reverse‑engineering purposes only.

use std::{io::{Write, stdin, stdout}, thread::sleep, time::Duration};

fn main() {
    let password = String::from("SECRET");

    loop {
        let mut input: String = String::new();

        print!("Enter your password: ");
        stdout().flush().expect("Failed to flush stdout!");

        stdin()
            .read_line(&mut input)
            .expect("Read error!");

        let input: &str = input.trim();

        if password == input {
            println!("Correct password!");
            sleep(Duration::from_secs(20));
            break;
        } else {
            println!("Password incorrect!");
        }
    }
}
```
<br>

<img width="998" height="843" alt="image" src="https://github.com/user-attachments/assets/a557d1f7-43d9-4ba1-8a42-643e87c33f68" />
<br><br>

### 2.3 In-depth Analysis

#### Imports

```rust
use std::{io::{Write, stdin, stdout}, thread::sleep, time::Duration};
```

This line imports functionality from the Rust standard library:

**stdin()** - reads user input from the terminal\
**stdout()** - writes output to the terminal\
**Write** - enables the .flush() method\
**sleep()** - pauses program execution\
**Duration** - defines how long the program sleeps

## 3. Reverse Engineering

## 4. Protection

## 5. Resources
