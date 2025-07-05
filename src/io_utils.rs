//! Input/Output utility functions module.
//!
//! This module provides utility functions for handling user input and output
//! operations. Currently, it focuses on user confirmation prompts but may
//! be extended with additional I/O utilities in the future.

use std::io::{self, Write};

/// Ask the user for a yes/no confirmation.
///
/// This function displays a prompt to the user and waits for a response.
/// It accepts 'Y' or 'N' (case insensitive) as valid responses and will
/// continue prompting until a valid response is received.
///
/// # Arguments
///
/// * `prompt` - The message to display to the user
///
/// # Returns
///
/// Returns `true` if the user confirms (enters 'Y'), `false` if they decline (enters 'N').
///
/// # Behavior
///
/// - The function will loop until a valid response is received
/// - Input is case-insensitive ('y', 'Y', 'n', 'N' are all valid)
/// - Invalid input will show an error message and prompt again
/// - IO errors during input reading will be displayed but won't crash the program
///
/// # Examples
///
/// ```
/// use todo_rs::io_utils::ask_user_confirmation;
///
/// let confirmed = ask_user_confirmation("Do you want to continue? (Y/N): ");
/// if confirmed {
///     println!("User confirmed!");
/// } else {
///     println!("User declined.");
/// }
/// ```
///
/// # Panics
///
/// This function will panic if stdout cannot be flushed, which typically
/// indicates a serious system-level issue.
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
