



#[cfg(test)]
mod tests {
    use std::io::BufRead;

    use crate::utils::read_file;

    #[test]
    fn test_read_line() {
        let file_path = "/Users/sox/CODE/minigrep/Cargo.toml".to_string();
        let reader = read_file(file_path).unwrap();

        for i in reader.lines() {
            println!("{}", i.unwrap());
        }
    }
    
}