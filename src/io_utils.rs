use std::io::{self, Write};

pub fn ask_user_confirmation(prompt: &str) -> bool {
    loop {
        print!("{prompt}");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("Failed to read input: {e}");
            continue;
        }

        match input.trim().to_ascii_uppercase().as_str() {
            "Y" => return true,
            "N" => return false,
            _ => {
                println!("Please enter 'Y' or 'N'.");
            }
        }
    }
}
