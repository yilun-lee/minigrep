



#[cfg(test)]
mod tests {
    // from lib
    use crate::grep::matcher::RegexMatcher;
    use crate::utils::{read_file, FileReader};
    use crate::main_loop;
    use crate::grep::handler::{LineReplacer, LineExtractor, LineMatcher};
    use crate::argparse::MiniGrepArg;

    #[test]
    fn test_read_line() {
        let file_path = "/Users/sox/CODE/minigrep/Cargo.toml".to_string();
        let lines = read_file(file_path).unwrap();

        for i in lines {
            println!("{}", i.unwrap());
        }
    }


    #[test]
    fn test_match() {
        let my_re = RegexMatcher::new(r"AA.+", true).unwrap();
        
        let my_line_replacer = LineMatcher {
            matcher: Box::new(my_re),
        };

        let file_path = String::from("/Users/sox/CODE/minigrep/example/test.txt");
        let file_reader = FileReader::new(file_path, 2, 2).unwrap();

        main_loop(file_reader, my_line_replacer).unwrap();

    }

    #[test]
    fn test_extract() {
        let my_re = RegexMatcher::new(r"[^0-9]+AA", true).unwrap();
        
        let my_line_replacer = LineExtractor {
            matcher: Box::new(my_re),
        };

        let file_path = String::from("/Users/sox/CODE/minigrep/example/test.txt");
        let file_reader = FileReader::new(file_path, 0, 0).unwrap();

        main_loop(file_reader, my_line_replacer).unwrap();

    }

    #[test]
    fn test_replace() {
        let my_re = RegexMatcher::new(r"AA", true).unwrap();
        
        let my_line_replacer = LineReplacer {
            matcher: Box::new(my_re),
            substitute: "BB",
            times: 0,
        };


        let file_path = String::from("/Users/sox/CODE/minigrep/example/test.txt");
        let file_reader = FileReader::new(file_path, 2, 2).unwrap();

        main_loop(file_reader, my_line_replacer).unwrap();

    }


    #[test]
    fn test_argparse() {

        let mystr: String = String::from(
            "minigrep -i \"[A-Z]+:[0-9\\.]+$\" --A 3 -B 4 /Users/sox/CODE/minigrep/example/test.txt");
            let my_cmd_iter = mystr.split(" ").map(|a| a.to_owned()).into_iter();
        
        let my_arg = MiniGrepArg::new(my_cmd_iter).unwrap();
        println!("{:#?}",my_arg);

    }

}

// cargo test -p grep test_loop  -- --show-output