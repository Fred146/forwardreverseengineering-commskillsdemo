# Forward / Reverse Engineering Demo

<br>

<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/afc67647-b6f8-475f-a682-c41f9e9b1e92" />

(Image courtesy of [Triple Infotech](https://tripleinfotech.com/2024/08/29/reverse-vs-forward-engineering-key-differences-and-when-to-choose))

<br><br>

## 1. Navigation

A table of contents can be accessed by clicking the appropriate button in the top-right corner of this document.

<img width="911" height="304" alt="{069D2F70-2685-499E-AC15-E95CD3487171}" src="https://github.com/user-attachments/assets/da2da1f4-9bf7-49ef-9845-34a6c1535282" />

<br><br>

## 2. Contact
Feel free to open a ticket if you experience any difficulties following the instructions below.

<br><br>

## 3. Forward Engineering
In this demo, we use [**Rust**](https://rust-lang.org) as the programming language and [**Visual Studio Code**](https://code.visualstudio.com) as the source code editor. You are free to use any platform or tools of your choice, as the outcome will be very similar. You may also skip this chapter entirely if you are only interested in the reverse-engineering part; an executable binary will be provided.

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

Prints "Password incorrect!" to the console when a previous condition is false.

<br><br>

### 3.4 Summary

- Prompts the user for a password
- Compares input to a hard‑coded value
- Repeats until the correct password is entered
- Waits 20 seconds before exiting

<br>

## 4. Reverse Engineering

We will use the debugger [**x64dbg**](https://x64dbg.com) to reverse engineer our application. If you want the memory addresses to match those shown in the screenshots, you may use the precompiled executable located at **target/release/commskillsdemo.exe** in this repository.

<br>

### 4.1 Setup

Open **commskillsdemo.exe** by double‑clicking it. Enter any text to confirm that the input is rejected. Take note of the error message, as it will be important later.

<br>

<img width="979" height="101" alt="image" src="https://github.com/user-attachments/assets/9417124b-2251-4cc5-abb7-62fb61f5ec23" />

<br><br>

Open **x64dbg** located at **release\x64\x64dbg.exe**. Once the debugger is running, go to the **File** menu and select **Attach**. In the list of processes, choose **commskillsdemo.exe** and click **Attach** to proceed.

<br>

<img width="802" height="432" alt="image" src="https://github.com/user-attachments/assets/402de5fd-bd97-4620-9d12-4c8606e7a98a" />

<br><br>

### 4.2 Code Location

Your debugger window should now look like this:

<img width="870" height="632" alt="image" src="https://github.com/user-attachments/assets/e78f68bf-4ba3-4ee3-a57a-06d2894e596b" />

<br><br>

It might seem complicated at first, but the most important section is the top-left code window. In order, it shows you:

- The **RIP** (Return Instruction Pointer) and lines/arrows indicating code jumps
- **Memory address locations**
- **CPU opcodes** (operation codes)
- **Assembly instructions**

Our next goal is to locate the section of code that triggers the error message. By identifying this location, we can attempt to modify the assembly instructions so that the program always follows the **success path**, regardless of the input provided.

One way to accomplish this is by performing a string search. Right‑click anywhere in the code view to open the context menu, then navigate to:

**Search for → All Modules → String references**

This will open the **References** window, which displays all locations in the binary where string literals are used. In the search input box at the bottom left, enter the error message:

```
Password incorrect!
```

You will then be presented with the memory address of the code that references this string. Double‑click the entry to navigate directly to the corresponding code location.

<img width="870" height="632" alt="image" src="https://github.com/user-attachments/assets/17574bb0-0515-41e5-8660-cb2dbb4cd575" />

<img width="870" height="632" alt="image" src="https://github.com/user-attachments/assets/65b2d5bb-e7a3-40af-92c7-8b0de5e627b4" />

<br><br>

### 4.3 Code Reversing - Else Elimination

We land at the memory address **00007FF7343A169A**, which is where the error message we want to avoid appears. By examining the lines above this one, we can see the broader context of what happens before this point, ultimately leading to the execution of the line we want to prevent.

<img width="916" height="100" alt="image" src="https://github.com/user-attachments/assets/84e0ad9d-a8e8-493c-9059-fa02e223fa9d" />

<br><br>

If we take a closer look we can see that the following line jumps to our error message:

```asm
00007FF7343A167C | 75 1C                    | jne commskillsdemo.7FF7343A169A         |
```

<br>

This corresponds to the **else** clause in Rust. **jne** stands for "jump if not equal"; in other words, if the user input does not match the password, an error message is displayed. We want to prevent this from happening. In a binary file, you cannot simply remove lines of code, as this would cause the subsequent code to “fall down” and change memory addresses. However, you can overwrite this code with a special operation called **NOP** (no operation). This effectively eliminates the instruction while keeping the file structure intact. To do this, right-click on the line of code, go to **Binary**, and then select **Fill with NOPs**.

<img width="957" height="110" alt="image" src="https://github.com/user-attachments/assets/aaf6412a-e8b3-4c91-8d0b-983ba689b3f5" />

<br><br>

Job done, right? We’ve effectively removed the else statement. However, as you can see below, there’s still some work to be done.

<img width="979" height="130" alt="image" src="https://github.com/user-attachments/assets/a31530a9-3b3c-4e7a-9f77-dd575ac7b0af" />

<br><br>

### 4.4 Code Reversing - If Condition

Our code will no longer perform the jump, but due to the way our Rust source code is translated into assembly, it still roughly follows the same path and ends up at the error message.

This is what the piece of code does, and the first step has already been eliminated by our previous actions:

- Jump to the error message if the condition is false.
- If the else jump is not taken, execution continues; a few lines later, a jump occurs if the condition is true.

<br>

```asm
00007FF7343A167C | 90                       | nop                                     |
00007FF7343A167D | 90                       | nop                                     |
00007FF7343A167E | 8B16                     | mov edx,dword ptr ds:[rsi]              | rsi:RtlSetThreadSubProcessTag+A50
00007FF7343A1680 | 331408                   | xor edx,dword ptr ds:[rax+rcx]          |
00007FF7343A1683 | 44:0FB746 04             | movzx r8d,word ptr ds:[rsi+4]           | rsi+04:RtlSetThreadSubProcessTag+A54
00007FF7343A1688 | 6644:334408 04           | xor r8w,word ptr ds:[rax+rcx+4]         |
00007FF7343A168E | 41:0FB7C0                | movzx eax,r8w                           |
00007FF7343A1692 | 09D0                     | or eax,edx                              |
00007FF7343A1694 | 0F84 76010000            | je commskillsdemo.7FF7343A1810          |
00007FF7343A169A | 48:8D05 0FBA0900         | lea rax,qword ptr ds:[7FF73443D0B0]     | 00007FF73443D0B0:&"Password incorrect!\n"
```

The last step is being executed at memory location **00007FF7343A1694**, which contains a **je** instruction (jump if equal). This instruction corresponds to the **if** condition in the Rust code, which executes this code only if the user input matches the password. We want it to always execute this code, regardless of the condition. In Rust, you can achieve this by replacing the condition with the keyword **true**. In assembly, we can achieve the same effect by changing **je** to **jmp**, which always jumps unconditionally.

Right-click this line of code and select the **Assemble** option. Sometimes **je** is also represented as **jz** (jump if zero), which at a lower level checks whether a value is zero or not. Change this **jz** instruction to **jmp** and press **OK**. The **Assemble** window will remain open but move to the next line. Since we are done modifying the code, you can now press **Cancel** to close it.

<img width="922" height="109" alt="image" src="https://github.com/user-attachments/assets/3e717590-0186-4581-af00-a11c553a8dec" />

<br><br>

If we try an incorrect password again we can now see that we succeed to enter the system.

<img width="979" height="165" alt="image" src="https://github.com/user-attachments/assets/3837fe67-c9f6-44ca-abc3-c00d1cfec65e" />

<br><br>

## 5. Protection

[VMProtect Software](https://vmpsoft.com)

<br>

## 6. Resources

[W3Schools Rust Tutorial](https://w3schools.com/rust)

[Game Hacking Academy](https://gamehacking.academy)
