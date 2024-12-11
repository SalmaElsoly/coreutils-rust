use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short='n', default_value_t = false)]
    omit_new_line_trainling: bool,

    args: String,
}

fn main(){
    let args = Args::parse();
    if args.omit_new_line_trainling{
        print!("{}", args.args);
        return;
    }
    println!("{}", args.args);
}