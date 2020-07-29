use std::io;

fn clean_str<'a>(s:&'a String) -> &'a str {
    &s[..].trim()
}

fn ask(prompt:&str) -> u8 {
    println!("{}", prompt);
    let mut question = String::new();
    io::stdin().read_line(&mut question).expect("leesfout");
    u8::from_str_radix(clean_str(&question), 10).expect("Geen nummer")
}

fn main() {
    let rounds = ask("Hoeveel rondes?");
    let players = ask("Hoeveel spelers?");
    let time = ask("Hoeveel tijd in minuten?");
    println!("Maximum speeltijd: {} minuten.", rounds * players * time);
}
