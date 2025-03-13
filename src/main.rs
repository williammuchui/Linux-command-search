use clap::{Parser, Subcommand};
use std::env::var;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::process::exit;

#[derive(Parser, Debug)]
#[command(version,long_about=None, about)]
struct Cli {
    #[command(subcommand)]
    command: Args,
}

#[derive(Subcommand, Debug)]
enum Args {
    /// List all available commands
    List,

    /// Search for a specific command
    Search {
        /// Command to search
        needle: String,

        /// Ignore case or search exact command
        #[clap(short = 'i')]
        ignorecase: bool,
    },
}

enum Ignorecase {
    True,
    False,
}

fn main() {
    let arguments = Cli::parse();

    draw_commands_ascii();

    match arguments.command {
        Args::List => handle_list_argument(),
        Args::Search { needle, ignorecase } => {
            if ignorecase {
                handle_search_command(&needle, Ignorecase::True);
            } else {
                handle_search_command(&needle, Ignorecase::False);
            }
        }
    }
}

fn handle_list_argument() {
    let filepath = get_file_path();
    if let Some(contents) = read_file(&filepath) {
        let lines: Vec<&str> = contents.lines().collect();
        println!("AVAILABLE LINUX COMMANDS:");
        for line in lines {
            print_contents(line);
        }
    } else {
        exit_with_error("Failed to read commands from file.");
    }
}

fn get_file_path() -> String {
    let home_dir = var("HOME").unwrap_or_else(|_| String::from("."));
    let mut path = PathBuf::from(home_dir);
    path.push(".commands/linux");
    path.to_str().unwrap().to_string()
}

fn read_file(filepath: &str) -> Option<String> {
    match File::open(filepath) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut contents = String::new();
            if let Err(e) = reader.read_to_string(&mut contents) {
                exit_with_error(&format!("Failed to read the file: {}", e));
                return None; // This line won't be reached, but is needed to satisfy the return type.
            }
            Some(contents)
        }
        Err(e) => {
            exit_with_error(&format!("Error opening file: {}", e));
            None
        }
    }
}

fn handle_search_command(needle: &str, ignorecase: Ignorecase) {
    let filepath = get_file_path();
    if let Some(haystack) = read_file(&filepath) {
        let found: Vec<&str> = match ignorecase {
            Ignorecase::True => haystack
                .lines()
                .filter(|line| line.to_lowercase().contains(&needle.to_lowercase()))
                .collect(),
            Ignorecase::False => haystack
                .lines()
                .filter(|line| line.contains(needle))
                .collect(),
        };

        if found.is_empty() {
            println!("NO COMMAND FOUND");
            return;
        }
        let total = found.len();
        println!(
            "{} MATCHING COMMAND{}",
            total,
            if total == 1 { "" } else { "S" }
        );
        println!("(*) MEANS THE COMMAND RUN BY ROOT USER");
        for line in found {
            print_contents(line);
        }
    }
}

fn exit_with_error(e: &str) {
    eprintln!("ERROR: \n{}", e);
    exit(1);
}

fn print_contents(contents: &str) {
    println!("{}", contents);
}

fn draw_commands_ascii() {
    let text = "
 ██████╗ ██████╗ ███╗   ███╗███╗   ███╗ █████╗ ███╗   ██╗██████╗ ███████╗
██╔════╝██╔═══██╗████╗ ████║████╗ ████║██╔══██╗████╗  ██║██╔══██╗██╔════╝
██║     ██║   ██║██╔████╔██║██╔████╔██║███████║██╔██╗ ██║██║  ██║███████╗
██║     ██║   ██║██║╚██╔╝██║██║╚██╔╝██║██╔══██║██║╚██╗██║██║  ██║╚════██║
╚██████╗╚██████╔╝██║ ╚═╝ ██║██║ ╚═╝ ██║██║  ██║██║ ╚████║██████╔╝███████║
 ╚═════╝ ╚═════╝ ╚═╝     ╚═╝╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═════╝ ╚══════╝
";
    println!("{}", text);
}
