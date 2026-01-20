use clap::{Parser, Subcommand};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

const VERSION: &str = "\
\n<------------------------------>
Fillet - v0.1.0
Copyright (c) 2025 Luna Moonlit Noir

A command line file editor and organizer tool.
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
    /// Read and print the contents of a file.
    Read {
        file: String,

        #[arg(short, long)]
        /// Read a specific line.
        line: Option<usize>,
    },

    /// Write to a file.
    Write {
        file: String,
        content: String,

        #[arg(short, long)]
        /// Replace the entire contents of the file.
        overwrite: bool,

        #[arg(short, long)]
        /// Add to the end of the file.
        append: bool,

        #[arg(short, long)]
        /// Replace a specific line.
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
            if let Some(l) = line {
                let content_str = fs::read_to_string(&file).unwrap_or_else(|e| {
                    eprintln!("Error, unable to read file '{}': {}", &file, e);
                    std::process::exit(1);
                });
                let mut lines: Vec<String> = content_str.lines().map(str::to_string).collect();

                if l == 0 || l > lines.len() {
                    eprintln!("Error, line {} is out of range.", l);
                    std::process::exit(1);
                }

                lines[l - 1] = content;
                let updated_content = lines.join("\n");
                fs::write(&file, updated_content).unwrap_or_else(|e| {
                    eprintln!("Error, unable to write to file '{}': {}", &file, e);
                    std::process::exit(1);
                });
            } else if overwrite {
                let result = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(file);

                match result {
                    Ok(mut ffile) => {
                        if let Err(e) = write!(ffile, "{}", content) {
                            eprintln!("Error, unable to write to file: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error, unable to open file: {}", e);
                    }
                }
            } else if append {
                match OpenOptions::new().append(true).create(true).open(&file) {
                    Ok(mut ffile) => {
                        if let Err(e) = write!(ffile, "\n{}", content) {
                            eprintln!("Failed to write to file: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error, could not open '{}' for appending: {}", &file, e);
                    }
                }
            } else {
                eprintln!("Error, missing argument for write.");
            }
        }
    }
}
