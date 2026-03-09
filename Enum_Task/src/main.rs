use std::io::{self, Write};
use std::process::Command;

enum FileOperation {
    List(String),            // Directory path
    Display(String),         // File path
    Create(String, String),  // File path and content
    Remove(String),          // File path
    Pwd,                     // Print working directory
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(path) => {
            let status = Command::new("ls")
                .arg(path)
                .status()
                .expect("Failed to execute ls");

            if !status.success() {
                println!("Error: Could not list directory.");
            }
        }

        FileOperation::Display(path) => {
            let status = Command::new("cat")
                .arg(path)
                .status()
                .expect("Failed to execute cat");

            if !status.success() {
                println!("Error: Could not display file.");
            }
        }

        FileOperation::Create(path, content) => {
            let command = format!("echo '{}' > {}", content, path);

            let status = Command::new("sh")
                .arg("-c")
                .arg(command)
                .status()
                .expect("Failed to create file");

            if status.success() {
                println!("File '{}' created successfully.", path);
            } else {
                println!("Error: Failed to create file.");
            }
        }

        FileOperation::Remove(path) => {
    let status = Command::new("rm")
        .arg(&path)
        .status()
        .expect("Failed to remove file");

    if status.success() {
        println!("File '{}' removed successfully.", path);
    } else {
        println!("Error: Could not remove file.");
    }
}

        FileOperation::Pwd => {
            let status = Command::new("pwd")
                .status()
                .expect("Failed to execute pwd");

            if !status.success() {
                println!("Error: Could not get working directory.");
            }
        }
    }
}

fn read_input(prompt: &str) -> String {
    let mut input = String::new();

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn main() {
    println!("Welcome to the File Operations Program!");

    loop {
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");

        let choice = read_input("Enter your choice (0-5): ");

        match choice.as_str() {
            "1" => {
                let dir = read_input("Enter directory path: ");
                let operation = FileOperation::List(dir);
                perform_operation(operation);
            }

            "2" => {
                let file = read_input("Enter file path: ");
                let operation = FileOperation::Display(file);
                perform_operation(operation);
            }

            "3" => {
                let file = read_input("Enter file path: ");
                let content = read_input("Enter content: ");
                let operation = FileOperation::Create(file, content);
                perform_operation(operation);
            }

            "4" => {
                let file = read_input("Enter file path: ");
                let operation = FileOperation::Remove(file);
                perform_operation(operation);
            }

            "5" => {
                let operation = FileOperation::Pwd;
                perform_operation(operation);
            }

            "0" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Invalid choice. Please enter a number between 0 and 5.");
            }
        }
    }
}