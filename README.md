# Forward / Reverse Engineering Demo

<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/afc67647-b6f8-475f-a682-c41f9e9b1e92" />
[(Image courtesy of Triple Infotech)](https://tripleinfotech.com/2024/08/29/reverse-vs-forward-engineering-key-differences-and-when-to-choose)

## 1. Navigation

A table of contents is provided by GitHub by clicking on the appropriate button at the top right section of this document.

<img width="751" height="279" alt="{4DE32AC5-EC36-4582-9F01-F32B73F34587}" src="https://github.com/user-attachments/assets/5eb85e2d-5726-42be-9b89-ab52d5f40e3c" />

<br>

## 2. Contact
Feel free to open a ticket if you experience any difficulties following the instructions below.

<br>

## 3. Forward Engineering
In this demo, we use **Rust** as the programming language and **Visual Studio Code** as the source code editor. You are free to use any platform or tools of your choice, as the outcome will be very similar. You may also skip this chapter entirely if you are only interested in the reverse-engineering part; an executable binary will be provided.

<br>

### 3.1 Project Initiation

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

### 3.2 Source Code

You will be presented with the following code:

<br>

```rust
fn main() {
    println!("Hello, world!");
}
```

<br>

You may remove all of these lines of code and replace them with the code below. Each section of the code will be explained in more detail afterward.

<br>

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

### 3.3 In-depth Analysis

#### 3.3.1 Imports

```rust
use std::{io::{Write, stdin, stdout}, thread::sleep, time::Duration};
```

This line imports functionality from the Rust standard library:

**stdin()** - reads user input from the terminal\
**stdout()** - writes output to the terminal\
**Write** - enables the **.flush()** method\
**sleep()** - pauses program execution\
**Duration** - defines how long the program sleeps

<br>

#### 3.3.2 Program Entry Point

```rust
fn main() {
```

The **main** function is the entry point of the program.

<br>

#### 3.3.3 Stored Password

```rust
let password = String::from("SECRET");
```

A **String** containing the hard‑coded password. User input is compared against this value.

<br>

#### 3.3.4 Infinite Loop

```rust
loop {
```

Creates an infinite loop that continues until a **break** statement is executed.

<br>

#### 3.3.5 Input Buffer

```rust
let mut input: String = String::new();
```

Allocates a mutable string to store user input.

<br>

#### 3.3.6 User Prompt

```rust
print!("Enter your password: ");
stdout().flush().expect("Failed to flush stdout!");
```

- Displays the prompt without a newline.
- **flush()** ensures the prompt appears immediately before input is read.

<br>

#### 3.3.7 Reading User Input

```rust
stdin()
    .read_line(&mut input)
    .expect("Read error!");
```

Reads a line of input from the user and stores it in **input**.\
The newline character (**\n**) is included.

<br>

#### 3.3.8 Trimming Input

```rust
let input: &str = input.trim();
```

Removes leading and trailing whitespace, including the newline character.

<br>

#### 3.3.9 Password Comparison

```rust
if password == input {
```

Compares the trimmed user input with the stored password.

<br>

#### 3.3.10 Correct Password Case

```rust
println!("Correct password!");
sleep(Duration::from_secs(20));
break;
```

- Prints a success message
- Pauses execution for 20 seconds
- Exits the loop and ends the program

<br>

#### 3.3.11 Incorrect Password Case

```rust
} else {
    println!("Password incorrect!");
}
```

<br><br>

### 3.4 Summary

- Prompts the user for a password
- Compares input to a hard‑coded value
- Repeats until the correct password is entered
- Waits 20 seconds before exiting

<br>

## 4. Reverse Engineering

## 5. Protection

## 6. Resources
