mod cli;
mod github;

use clap::Parser;
use cli::Cli;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, terminal,
};
use std::error::Error;
use std::io::{self, Write};

fn prompt_input_interactive(prompt: &str) -> io::Result<String> {
    // Print the prompt
    print!("{}", prompt);
    io::stdout().flush()?;

    // Enable raw mode to get individual keystrokes
    terminal::enable_raw_mode()?;

    let mut input = String::new();
    let mut cursor_position = 0;

    loop {
        // Wait for a key event
        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event::read()?
        {
            match code {
                // Enter key submits the input
                KeyCode::Enter => {
                    break;
                }
                // Backspace deletes the character before the cursor
                KeyCode::Backspace => {
                    if cursor_position > 0 {
                        input.remove(cursor_position - 1);
                        cursor_position -= 1;
                    }
                }
                // Delete key removes the character at the cursor position
                KeyCode::Delete => {
                    if cursor_position < input.len() {
                        input.remove(cursor_position);
                    }
                }
                // Left arrow moves cursor left
                KeyCode::Left => {
                    if cursor_position > 0 {
                        cursor_position -= 1;
                    }
                }
                // Right arrow moves cursor right
                KeyCode::Right => {
                    if cursor_position < input.len() {
                        cursor_position += 1;
                    }
                }
                // Home key moves to the start of the input
                KeyCode::Home => {
                    cursor_position = 0;
                }
                // End key moves to the end of the input
                KeyCode::End => {
                    cursor_position = input.len();
                }
                // Ctrl+C exits
                KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                    // Disable raw mode before exiting
                    terminal::disable_raw_mode()?;
                    println!();
                    return Err(io::Error::new(io::ErrorKind::Interrupted, "Interrupted"));
                }
                // Regular character input
                KeyCode::Char(c) => {
                    input.insert(cursor_position, c);
                    cursor_position += 1;
                }
                _ => {}
            }

            // Clear the current line and redraw the prompt and input
            execute!(
                io::stdout(),
                cursor::MoveToColumn(0),
                terminal::Clear(terminal::ClearType::CurrentLine)
            )?;
            print!("{}{}", prompt, input);

            // Move cursor to the correct position
            execute!(
                io::stdout(),
                cursor::MoveToColumn((prompt.len() + cursor_position) as u16)
            )?;

            io::stdout().flush()?;
        }
    }

    // Disable raw mode
    terminal::disable_raw_mode()?;
    println!(); // Add a newline after input

    Ok(input)
}

fn prompt_non_empty_input_interactive(prompt: &str) -> io::Result<String> {
    loop {
        let input = prompt_input_interactive(prompt)?;
        if !input.is_empty() {
            return Ok(input);
        }
        println!("Input cannot be empty. Please try again.");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Ask for repository name (required)
    let name = prompt_non_empty_input_interactive("Enter repository name: ")?;

    // Ask for repository description (optional)
    let description_input = prompt_input_interactive("Enter repository description (optional): ")?;
    let description = if description_input.is_empty() {
        None
    } else {
        Some(description_input)
    };

    println!("Creating a new GitHub repository: {}", name);

    // Ask user if they want a public or private repository
    let private_input =
        prompt_input_interactive("Do you want the repository to be private? (Y/n): ")?;
    let private = private_input.is_empty() || private_input.to_lowercase() != "n";

    let visibility = if private { "private" } else { "public" };
    println!("Creating a {} repository...", visibility);

    match github::create_github_repo(&cli.token, &name, &description, private).await {
        Ok(url) => {
            println!("\n✅ Repository created successfully!");
            println!("Repository URL: {}", url);
            println!("\nYou can clone it with:");
            // Convert HTTPS URL to SSH format for git clone
            let ssh_url = url.replace("https://github.com/", "git@github.com:").replace(".git", "") + ".git";
            println!("git clone {}", ssh_url);
        }
        Err(e) => {
            eprintln!("❌ Error creating repository: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
