use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Selina Liu",
    about = "Calculate the result of a reverse polish notation expression"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Selina Liu")]
    Evaluate {
        #[clap(short, long)]
        input: String,
    },
}

// write a main function that parses the command line arguments, form a vector of strings by parsing with delimiter "," and then process the vector of strings
fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Evaluate { input }) => {
            let mut stack: Vec<i32> = Vec::new();
            let mut result: i32 = 0;
            let tokens: Vec<&str> = input.split(",").collect();
            for token in tokens {
                if token == "+" {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    result = a + b;
                    stack.push(result);
                } else if token == "-" {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    result = b - a;
                    stack.push(result);
                } else if token == "*" {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    result = a * b;
                    stack.push(result);
                } else if token == "/" {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    result = b / a;
                    stack.push(result);
                } else {
                    let a = token.parse::<i32>().unwrap();
                    stack.push(a);
                }
            }
            println!("The evaluated result is: {}", result);
        }
        None => println!("No subcommand was used"),
    }
}

