



#[cfg(test)]
mod tests {
    // from lib
    use crate::utils::glober::PathGlober;
    use crate::argparse::MiniGrepArg;

    #[test]
    fn test_read_line() {
        let my_path = PathGlober::new("/Users/sox/CODE/minigrep/grep/*", true).unwrap();
        
        for file_path in my_path {
            println!("{}",file_path);
        }
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