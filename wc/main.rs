use clap::Parser;
use std::{fs::File, io::Read};
use log::error;

#[derive(Parser)]
struct Args{
    #[arg(short,default_value_t=false)]
    words: bool,
    #[arg(short,default_value_t=false)]
    lines: bool,
    #[arg(short,default_value_t=false)]
    chars: bool,

    file: String
}

fn main(){
    let args = Args::parse();
    let mut file = match File::open(args.file){
        Ok(file)=>file,
        Err(_)=>{
            error!("error occured while openning the file");
            std::process::exit(1);
        }
    };
    let mut file_content = String::new();
    match file.read_to_string(&mut file_content){
        Ok(str)=>str,
        Err(_)=>{
            error!("could not read file");
            std::process::exit(1);
        }
    };
    let file_lines: Vec<&str> = file_content.split_terminator("\n").collect();
    let file_words: Vec<&str> = file_content.split_whitespace().collect();
    let file_chars: Vec<char> = file_content.chars().collect();

    if args.words{
        println!("Words: {}", file_words.len());
    }
    if args.lines{
        println!("Lines: {}", file_lines.len());
    }
    if args.chars{
        println!("Chars: {}", file_chars.len());
    }

    if !args.words && !args.lines && !args.chars{
        println!("Words: {}", file_words.len());
        println!("Lines: {}", file_lines.len());
        println!("Chars: {}", file_chars.len());
    }

}