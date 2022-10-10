pub mod Configure {
    pub struct Config {
        pub search_term :String,
        pub file_path : String
    }
    impl Config {
        pub fn check_args(args:Vec<String>) -> Result<Config, &'static str> {
            let mut new_config = Config{search_term:String::from(""), file_path:String::from("")};
            if args.len()!= 5 {
                return Err("you need to provide 4 arguments")
            }
            for (i,arg) in args.iter().enumerate() {
                if i%2 != 0 {
                    match arg.as_str() {
                        "-f" => new_config.file_path = args[i+1].clone(),
                        "-s" => new_config.search_term = args[i+1].clone(),
                        _ => println!("nothing found")
                    }
                }
            }
            Ok(new_config)
        }
        pub fn search<'a>(search_string:&'a String, file_contents:&'a String) -> Vec<&'a str> {
            file_contents
                .lines()
                .filter(|line| line.contains(search_string))
                .collect()
        }
    }
}



#[cfg(test)]
mod config_tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_check_args() {
        let args = vec![String::from("BinaryExe"), String::from("-s"), String::from("strange"), String::from("-f"), String::from("./file.txt")];
        let fn_output = Configure::Config::check_args(args).ok().unwrap();
        assert_eq!(vec!["BinaryExe", "-s", "strange", "-f", "./file.txt"], fn_output);

    } 
    #[test]
    fn test_search() {
        let file_contents = String::from("Big apples are good.\n I love seeds. \nThey are fantastic.");
        let search_string = String::from("good");
        let line_matches = Configure::Config::search(&search_string, &file_contents);

        assert_eq!(1, line_matches.len());
    }
}