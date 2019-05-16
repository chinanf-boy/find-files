#[macro_use]
extern crate error_chain;
extern crate glob;

use glob::glob;
use std::path::PathBuf;

error_chain! {
    foreign_links {
        Glob(glob::GlobError);
        Pattern(glob::PatternError);
    }
}

pub fn find_files() -> Result<Vec<PathBuf>> {
    let mut res: Vec<PathBuf> = Vec::new();
    for entry in glob("**/*.md").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => res.push(path),
            Err(e) => println!("{:?}", e),
        }
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::find_files;
    use std::path::PathBuf;

    #[test]
    fn it_works() {
        let path: PathBuf = [r"test.md"].iter().collect();
        assert_eq!(find_files().ok(), Some(vec![path]));
    }
}
