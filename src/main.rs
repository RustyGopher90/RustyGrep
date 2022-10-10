use std::{env::{self}, fs, process};
use rustyGrep::Configure::Config;

fn main() {
    let args = Config::check_args(env::args().collect());
    if args.as_ref().is_err(){
        println!("{}", args.as_ref().err().unwrap());
        process::exit(1);
    }
    let values:Config = args.ok().unwrap();
    
    let file_contents = fs::read_to_string(&values.file_path);
    if file_contents.is_err() {
        println!("{}", file_contents.as_ref().err().unwrap())
    }
    let unwrapped_file_contents = file_contents.ok().unwrap();
    let matches = Config::search(&values.search_term, &unwrapped_file_contents);
    if matches.len() < 1 {
        println!("There are no matches for: {}, in the file: {}", &values.search_term, &values.file_path)
    }else{
        for file_match in matches {
            println!("{:?}", file_match)
        }
    }
    
}
