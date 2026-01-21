use std::io::{self, Write};
use std::env;
use std::path::Path;
use std::process::{Command, Output};

fn wait_for_enter() {
    println!("Press Enter to continue...");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}

fn handle_command_output(output: Output) {
    if !output.status.success() {
        eprintln!("Command failed:");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    } else {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

fn cd() {
    let target_dir = Path::new("VideoEditing2026");
    if target_dir.exists() {
        env::set_current_dir(target_dir).expect("Failed to change directory");
    }
}

// Function for repository cloning
fn clone(repo_url: &str) {
    let clone_cmd = Command::new("git")
        .arg("clone")
        .arg(repo_url)
        .output()
        .unwrap();

    handle_command_output(clone_cmd);
}

// Function to fetch changes from the remote repository
fn fetch() {
    cd();
    let fetch_cmd = Command::new("git")
        .arg("fetch")
        .arg("origin")
        .output()
        .expect("Failed to execute git fetch command");
    
    handle_command_output(fetch_cmd);
}

// Function to pull changes with rebase
fn pull() {
    cd();
    fetch();
    let pull_cmd = Command::new("git")
        .arg("merge")
        .arg("origin/main")
        .output()
        .expect("Failed to execute git pull command");

    handle_command_output(pull_cmd);
}

// Function to stage, commit, and push changes
fn push() {
    cd();
    let add_cmd = Command::new("git")
        .arg("add")
        .arg(".") // Stage all files respecting .gitignore
        .output()
        .expect("Failed to execute git add command");
    handle_command_output(add_cmd);

    let name = get_user_name();
    let commit_message = format!("Update from {}", name);

    let commit_cmd = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .output()
        .expect("Failed to execute git commit command");
    handle_command_output(commit_cmd);

    let push_cmd = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("main")
        .output()
        .expect("Failed to execute git push command");
    handle_command_output(push_cmd);
}

// Helper function to get and validate user input
fn get_user_name() -> String {
    let mut name = String::new();
    loop {
        print!("What is your name? (Without Space Please): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut name).unwrap();
        let name = name.trim();
        if !name.is_empty() {
            return name.to_string();
        }
        println!("Name cannot be empty. Please try again.");
    }
}

// Introductory function with a friendly message
fn intro(repo_url: &str) {
    println!("Hello Strawhat!");
    println!("Let me introduce what you need to do to avoid conflicts when collaborating:");
    println!("\n1. Clone the repository.");
    println!("   Don't worry, I'll do that for you now.");
    
    clone(repo_url);
    println!("\n   You should now find a new folder named 'VideoEditing2026' in the current directory.");
    println!("   WARNING: Inform others before starting your work to avoid conflicts,");
    println!("   as non-text files cannot be merged by Git.\n");
    
    println!("2. Import the project to DaVinci Resolve and start working on your part.");
    println!("   Remember to EXPORT the project after completing your work (not just SAVE).\n");
    
    println!("3. Close this program while working. When you're done, re-run this program to upload your changes.");
    println!("4. Always run this program to DOWNLOAD the latest version of files before working again.\n");
    
    println!("This program simplifies syncing your work with others, so you don't need to know Git commands.");
    println!("For questions or assistance, contact Cloumy074 on Discord. Thank you!\n");
}

fn main() {
    let repo_url = "https://github.com/Vanier-Robotics/VideoEditing2026.git";

    println!("Hello Strawhat! Please tell me what you want to do:
    1. This is my first use.
    2. I need to download the working files.
    3. I want to upload my work.
    4. Exit.");

    'user_input: loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        match io::stdin().read_line(&mut choice) {
            Ok(_n) => {}
            Err(error) => {
                println!("error: {error}");
                break 'user_input;
            },
        }

        match choice.trim() {
            "1" => {
                intro(repo_url);
                wait_for_enter();
            }
            "2" => {
                pull();
                wait_for_enter();
            }
            "3" => {
                push();
                wait_for_enter();
            }
            "4" => {
                println!("Exiting. See you later!");
                break 'user_input;
            }
            _ => println!("Invalid choice. Please choose a valid option."),
        } 
    }
}
