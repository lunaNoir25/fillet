use clap::{Parser, Subcommand};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};

const VERSION: &str = "\
\n<------------------------------>
Fillet - v0.1.0
Copyright (c) 2025 Luna Moonlit Noir

A command line file editor and organizer tool.
Made for a school project. TwT
<------------------------------->
";

#[derive(Parser, Debug)]
#[command(author, version = VERSION, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Read {
        file: String,

        #[arg(short, long)]
        line: Option<usize>,
    },

    Write {
        file: String,
        content: String,

        #[arg(short, long)]
        overwrite: bool,

        #[arg(short, long)]
        append: bool,

        #[arg(short, long)]
        line: Option<usize>,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Read { file, line } => {
            if let Some(n) = line {
                let f = File::open(&file).expect("Error, unable to open file!");
                let reader = BufReader::new(f);

                #[allow(unused_variables)]
                match reader.lines().nth(n - 1) {
                    Some(Ok(content)) => println!("{}", content),
                    Some(Err(e)) => eprintln!("Error reading line {} -> {}", n, e),
                    none => eprintln!("Error, line {} out of range.", n),
                }
            } else {
                let contents = fs::read_to_string(file).expect("Error, unable to read file!");
                println!("{}", contents);
            }
        }

        Commands::Write {
            file,
            content,
            overwrite,
            append,
            line,
        } => {
            println!("Writing to {}", file);
            println!("Content: {}", content);

            if let Some(l) = line {
                println!("Write at line {}", l);
            } else if overwrite {
                println!("Overwrite");
            } else if append {
                println!("Append");
            } else {
                eprintln!("Error, missing argument for write.");
            }
        }
    }
}
