use std::io;
use std::io::prelude::*;

/// commandline pause
fn pause() {
    println!("Druk op een enter om verder te gaan...");
    io::stdin().read(&mut [0]).expect("oeps");
}

/// clean the string
fn clean_str<'a>(s:&'a String) -> &'a str {
    &s[..].trim()
}

/// ask for a number with prompt and return it
fn ask(prompt:&str) -> u8 {
    println!("{}", prompt);
    let mut question = String::new();
    io::stdin().read_line(&mut question).expect("leesfout");
    match u8::from_str_radix(clean_str(&question), 10) {
        Ok(answer) => answer,
        Err(_) => {
            println!("Voer een getal in");
            ask(prompt) // re-ask (seems to not leak memory)
        }
    }
}

fn main() {
    let rounds = ask("Hoeveel rondes?");
    let players = ask("Hoeveel spelers?");
    let time = ask("Hoeveel tijd in minuten?");
    println!("Maximum speeltijd: {} minuten.", rounds * players * time);
    pause();
}
