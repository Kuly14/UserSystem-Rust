use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome");
    let mut command = String::new();
    let command = loop {
        println!("To register new user press 1!");
        println!("To log in press 2!");
        match io::stdin().read_line(&mut command) {
            Ok(_) => (),
            Err(e) => {
                println!("Failed with error: {}", e);
                println!("Try again");
                continue;
            }
        };

        let command: u32 = match command.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Failed with error: {}", e);
                println!("Try again");
                continue;
            }
        };

        if command > 3 {
            println!("Command has to be either 1, 2 or 3");
            println!("Try again");
            continue;
        } else {
            break command;
        }
    };

    passwords::dispatch(&command)?;

    Ok(())
}
