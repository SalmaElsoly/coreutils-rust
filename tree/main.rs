use clap::Parser;
use std::fs;
use log::error;

#[derive(Parser)]
struct Args {
    #[arg(short='l', default_value_t = 1)]
    depth_level: u32,

    #[arg(short='d', default_value_t = false)]
    directories_only: bool,

    #[arg(default_value_t = String::from("."))]
    directory: String,
}

fn main(){
    let args = Args::parse();
    match print_tree(&args.directory,args.directories_only,0,args.depth_level){
        Ok(_)=>(),
        Err(err)=>{
            error!("could not print tree: {}", err);
            std::process::exit(1);
        }
    }
}

fn print_tree(directory: &str,directories_only: bool, current_level:u32,depth_level: u32)->std::io::Result<()>{
    if current_level >= depth_level{
       return Ok(());
    }

    for entry in fs::read_dir(directory)?{
        let entry = entry?;
        let path = entry.path();
        if path.is_dir(){
            println!("{}|__ {}", String::from("    ").repeat(current_level as usize), String::from(entry.file_name().to_str().unwrap()).trim_matches('"'));
            print_tree(path.to_str().unwrap(),directories_only,current_level+1,depth_level)?;
        }
        if path.is_file() && !directories_only{
            println!("{}|__ {}", String::from("    ").repeat(current_level as usize), String::from(entry.file_name().to_str().unwrap()).trim_matches('"'));
        }
    };
    Ok(())
}