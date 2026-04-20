
mod brain;

use std::io::{self, Write};
use brain::Brain;

fn main() {
    let mut bot = Brain::new();

    println!("╔══════════════════════════════════════╗");
    println!("║        Rust Conversational Bot        ║");
    println!("╚══════════════════════════════════════╝");
    println!("Bot: Hello! I'm here to chat, listen, and help where I can.");
    println!("     Ask me anything, share what's on your mind, or just talk.");
    println!("     (Type 'quit' or 'exit' to leave)\n");

    loop {
        print!("You: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!("\nBot: Goodbye!");
                break;
            }
            Ok(_) => {
                let text = input.trim();
                if text.eq_ignore_ascii_case("quit") || text.eq_ignore_ascii_case("exit") {
                    println!("Bot: It was great talking with you. Take care!");
                    break;
                }
                if !text.is_empty() {
                    let response = bot.process(text);
                    println!("Bot: {}\n", response);
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}