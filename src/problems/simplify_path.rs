// Given a file path, return the shortest possible file path
// (i.e. eliminate all ".." and ".")


use std::vec::Vec;

/// Use linux-style-paths
pub fn simplify_path(path: &String) -> String {
    let tokens: Vec<&str> = path.split("/").collect();
    let mut stack = Vec::with_capacity(tokens.len()); // len = tks.len() if already simple
    for token in tokens {
        match token {
            "" => continue, // ignore empty levels
            "." => continue, // ignore ".", it's useless
            ".." => {stack.pop();}, // go "up" by removing a level of depth
            _ => stack.push(token) // push level onto stack
        }
    }
    let final_path = stack
        .iter()
        .fold(
            "/".to_string(),
            |acc, x| acc + x + "/" // new = "/whatever/is/there/" + x + "/"
        );

    return final_path;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simplify_path() {
        assert_eq!(simplify_path(&"/Users/Joma/Documents/../Desktop/./..".to_string()), "/Users/Joma/");
        assert_eq!(simplify_path(&"/Users/Joma/Documents/..//Desktop/./..".to_string()), "/Users/Joma/");
        assert_eq!(simplify_path(&"./Users/Joma/Documents/../Desktop/./..".to_string()), "/Users/Joma/");
        assert_eq!(simplify_path(&"Users/Joma/Documents/../Desktop/./..".to_string()), "/Users/Joma/");
        assert_eq!(simplify_path(&"".to_string()), "/");
        assert_eq!(simplify_path(&"..".to_string()), "/");
    }

}
