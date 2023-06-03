#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    // from lib
    use super::super::gitclone::{GitClone, GitCloner};
    use super::super::{
        mygrep::{FileMatch, MiniGrepMatcher},
        GitGrepArg,
    };

    #[test]
    fn test_match_job() {
        let file_path = String::from("../../../example/test.txt");
        let expr = vec!["AA".to_string()];
        let my_mather = MiniGrepMatcher::new(expr, 1, 3);
        my_mather.match_job(file_path).unwrap();
    }

    #[test]
    fn test_git_clone() {
        let git_url = "https://github.com/yilun-lee/minigrep.git";
        let output_path = "/Users/sox/Downloads/minigrep";

        let mut git_cloner = GitCloner::new(vec![]);
        let (out, err) = git_cloner.git_clone(git_url, output_path).unwrap();

        println!("out: {}", out);
        println!("err: {}", err);
        assert!(Path::new(output_path).is_dir(), "{output_path} not existed");
        fs::remove_dir_all(output_path).unwrap();
    }

    #[test]
    fn test_clone_match() {
        let output_path = "/Users/sox/Downloads";
        let expr = vec!["Fn".to_string()];
        let mut clone_matcher = GitGrepArg::new(output_path.to_string(), expr, 1, 3);

        let git_url = "https://github.com/yilun-lee/minigrep.git";
        clone_matcher.clone_match(git_url).unwrap();

        let git_url = "https://github.com/yilun-lee/cocktail.git";
        clone_matcher.clone_match(git_url).unwrap();

        let git_url = "https://github.com/yilun-lee/minigrep.git";
        clone_matcher.clone_match(git_url).unwrap();

        clone_matcher.free_cache().unwrap();
    }
}
