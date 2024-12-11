use clap::Parser;
use std::{fs::File, io::Read};

#[derive(Parser)]
struct Args{
    #[arg(short,default_value_t=10)]
    number:usize,

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
    let file_content: Vec<&str> = file_content.split_terminator("/n").collect();
    let length: usize = match file_content.len().try_into(){
        Ok(len)=>len,
        Err(_)=> panic!("could not convert length to u64")
    };

    while args.number != 0 && length-args.number > 0{
        println!("{}", file_content[length-args.number])
    }
    
}
