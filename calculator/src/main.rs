//A command-line calculator
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Kahlia Hogg", about = "A simple calculator")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Kahlia Hogg")]
    Calc {
        #[clap(short, long)]
        x: f64,
        #[clap(short, long)]
        y: f64,
        #[clap(short, long)]
        op: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Calc { x, y, op }) => {
            if op == "+" {
                println!("Result: {:.2}", calculator::add(x, y));
            } else if op == "-" {
                println!("Result: {:.2}", calculator::subtract(x, y));
            } else if op == "/" {
                println!("Result: {:.2}", calculator::divide(x, y));
            } else if op == "x" {
                println!("Result: {:.2}", calculator::multiply(x, y));
            } else {
                println!("Invalid operator");
            }
        }
        None => println!("No command given"),
    }
}
