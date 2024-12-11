use clap::Parser;
use std::{fs::File, io::Read};

#[derive(Parser)]
struct Args{
    #[arg(short,default_value_t=10)]
    number:u64,

    file: String
}

fn main() {
    let args = Args::parse();
    let mut file = match File::open(args.file) {
        Ok(file)=>file,
        Err(_)=> panic!("error occured while openning the file")
    };
    let mut file_content = String::new();
    match  file.read_to_string(&mut file_content){
        Ok(str)=>str,
        Err(_)=>panic!("could not read file")
    };
    let file_lines: Vec<&str> = file_content.split_terminator("\n").collect();
    let lines: usize = match args.number.try_into() {
        Ok(lines)=>lines,
        Err(_)=> panic!("could not convert usize to u64")
    };
    let index = if lines > file_lines.len() {file_lines.len()}else{lines};

    for line in &file_lines[0..index]{
        println!("{line}");
    }
    
}
