use clap::Parser;
use std::{fs::File, io::Read};
use log::error;

#[derive(Parser)]
struct Args {
    #[arg(short, default_value_t = false)]
    numbered_lines: bool,

    file: String,
}

fn main() {
    let args = Args::parse();
    let mut file = match File::open(args.file) {
        Ok(file) => file,
        Err(_) => {
            error!("error occured while openning the file");
            std::process::exit(1);
        }
    };
    let mut file_content = String::new();
    match file.read_to_string(&mut file_content) {
        Ok(str) => str,
        Err(_) => {
            error!("could not read file");
            std::process::exit(1);
        },
    };
    let file_content: Vec<&str> = file_content.split_terminator("\n").collect();

    if args.numbered_lines {
        for (index, line) in file_content.iter().enumerate() {
            println!("{}    {line}", index + 1);
        }
        return;
    }

    for line in file_content {
        println!("{line}");
    }
}
