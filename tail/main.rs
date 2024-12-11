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
    let file_content: Vec<&str> = file_content.split_terminator("\n").collect();
    let length: u64 = match file_content.len().try_into(){
        Ok(len)=>len,
        Err(_)=> panic!("could not convert length to u64")
    };
    let mut i: u64= if length > args.number{args.number}else{length};

    while i != 0{
        let index = length.abs_diff(i);
        println!("{}", file_content[index as usize]);
        i -= 1;
    };
    
}
