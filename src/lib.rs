use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fs;
use std::io;
use std::io::prelude::*;
use std::process::Command;

#[derive(Hash)]
pub struct User {
    pub username: String,
    pub password: String,
}

pub enum UserStatus {
    None, 
    WrongPassword,
    Works,
}

pub fn dispatch(command: &u32) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        1 => {
            println!("Input your new username: ");
            let mut username = String::new();

            loop {
                io::stdin().read_line(&mut username)?;
                println!("Checking if the username already exists...");
                let exists = check_if_username_exists(&username);
                if exists == true {
                    Command::new("clear").status().unwrap();
                    println!("This username already exists");
                    println!("Try again: ");
                    continue;
                } else {
                    break;
                }
            }

            let mut password = String::new();
            println!("This username is free.");
            println!("Great! Now input your password: ");
            io::stdin().read_line(&mut password)?;
            println!("Registering new user...");
            register_user(&username, &password);
            println!("Do you want to log in?");
            let command = loop {
                println!("Press 1 for yes, press 2 for no");
                let mut command = String::new();
                io::stdin().read_line(&mut command)?;

                let command: u32 = command.trim().parse()?;
                if command > 2 {
                    Command::new("clear").status().unwrap();
                    println!("Command has to be either 1 or 2");
                    println!("Try again");
                    continue;
                } else {
                    break command;
                }
            };

            match command {
                1 => {
                    log_in(&username, &password);
                }
                2 => {
                    println!("Exiting session");
                    std::process::exit(1);
                }
                _ => std::process::exit(1),
            };
        }
        2 => {
            'outer: loop {
                let username = 'inner_one: loop {
                    println!("Username: ");
                    let mut username = String::new();
                    match io::stdin().read_line(&mut username) {
                        Ok(_) => (),
                        Err(e) => {
                            Command::new("clear").status().unwrap();
                            println!("Failed with error: {}", e);
                            println!("Try again");
                            println!("Username: ");
                            continue 'inner_one;
                        },
                    };
                    break 'inner_one username;
                };

                let password = 'inner_two: loop {
                    println!("Password: ");
                    let mut password = String::new();
                    match io::stdin().read_line(&mut password) {
                        Ok(_) => (),
                        Err(e) => {
                            Command::new("clear").status().unwrap();
                            println!("Failed with error: {}", e);
                            println!("Try again");
                            println!("Username: ");
                            continue 'inner_two;
                        }
                    }
                    break 'inner_two password;
                };

                match log_in(&username, &password) {
                    UserStatus::None => {
                        Command::new("clear").status().unwrap();
                        println!("This username is not registered!");
                        println!("Would you like to try again?");
                        'inner_three: loop {
                            println!("y/n");
                            let mut command = String::new();
                            match io::stdin().read_line(&mut command) {
                                Ok(_) => (),
                                Err(e) => {
                                    println!("Failed with this error: {}", e);
                                    println!("Try again: ");
                                    continue 'inner_three;
                                }
                            };

                            if command.trim() == String::from("y") {
                                break 'inner_three;
                            } else if command.trim() == String::from("n") {
                                println!("Exiting...");
                                std::process::exit(1);
                            } else {
                                println!("Invalid");
                                println!("Try again");
                                continue 'inner_three;
                            }
                        };
                    },
                    UserStatus::WrongPassword => {
                        println!("Wrong password try again");
                        continue 'outer;
                    },
                    UserStatus::Works => {
                        break 'outer;

                    }
                };
            };
        },
        _ => std::process::exit(1),
    };

    Ok(())
}

pub fn check_if_username_exists(username: &String) -> bool {
    let file = fs::read_to_string("data.txt").unwrap();
    for line in file.lines() {
        if line.trim() == username.trim() {
            return true;
        }
    }
    false
}

pub fn register_user(username: &String, password: &String) {
    let user = User {
        username: username.to_string(),
        password: password.to_string(),
    };
    let hash = calculate_hash(&user);

    let content = format!("{}\t {}", user.username, hash);

    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("data.txt")
        .unwrap();
    writeln!(file, "{}", content).unwrap();
    Command::new("clear").status().unwrap();
    println!("Success you can log in now");
}

pub fn log_in(username: &String, password: &String) -> UserStatus {
    let file = fs::read_to_string("data.txt").unwrap();
    let mut lines = file.lines();
    let mut passcode = String::new();
    let mut exists = String::new();
    while let Some(line) = lines.next() {
        if line.trim() == username.trim() {
            passcode.push_str(lines.next().unwrap());
            exists.push_str("EXISTS");
        }
    }

    let user = User {
        username: username.to_string(),
        password: password.to_string(),
    };

    let users_password = calculate_hash(&user);

    if passcode.is_empty() {
        return UserStatus::None;
    }

    let passcode: u64 = passcode.trim().parse().unwrap();

    if users_password == passcode {
        println!("Logged in successfuly!!!");
        return UserStatus::Works;
    } 

    if exists.is_empty() {
        return UserStatus::None;
    } else {
        println!("Wrong password");
        println!("Try again");
        return UserStatus::WrongPassword;
    }
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}


