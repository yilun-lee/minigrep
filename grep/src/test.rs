



#[cfg(test)]
mod tests {
    // from lib
    use crate::grep::matcher::RegexMatcher;
    use crate::utils::{FileReader, LinePrinter, PathGlober};
    use crate::main_loop;
    use crate::grep::handler::{LineReplacer, LineMatcher};
    use crate::argparse::MiniGrepArg;

    #[test]
    fn test_read_line() {
        let my_path = PathGlober::new("/Users/sox/CODE/minigrep/grep/*", true).unwrap();
        
        for file_path in my_path {
            println!("{}",file_path);
        }
    }

    #[test]
    fn test_match() {
        let my_re = RegexMatcher::new(r"AA.+", true).unwrap();
        
        let my_line_replacer = LineMatcher {
            matcher: Box::new(my_re),
        };

        let file_path: String = String::from("/Users/sox/CODE/minigrep/example/test.txt");
        let file_reader = FileReader::new(file_path, 2, 2).unwrap();
        let line_printer: LinePrinter = LinePrinter{ line_num_flag: true, file_path_flag: true, };

        //main_loop(file_reader, my_line_replacer, line_printer).unwrap();

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

        let line_printer: LinePrinter = LinePrinter{ line_num_flag: true, file_path_flag: true, };

        //main_loop(file_reader, my_line_replacer, line_printer).unwrap();

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