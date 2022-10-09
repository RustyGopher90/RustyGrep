pub mod Configure {

    pub struct Config {
        pub search_term : String,
        pub file_path : String
    }
    impl Config {
        pub fn get_arg_values(args:Vec<String>) -> Config {
            let search_term:String = args[2].to_string();
            let file_path:String = args[4].to_string();
            Config { search_term, file_path }
        }
        pub fn check_args(args:Vec<String>) -> Result<Vec<String>, &'static str> {
            if args.len()!= 5 {
                return Err("you need to provide 4 arguments")
            }
            if args[1] != "-s" {
                return Err("You need to pass a string first and then a File. \nExample -s somestring -f somefilepath.")
            }
            if args[3] != "-f" {
                return Err("You need to pass a string first and then a File. \nExample -s somestring -f somefilepath.")
            }
            return Ok(args)
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