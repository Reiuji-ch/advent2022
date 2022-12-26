use std::io::BufRead;
use std::path::PathBuf;
use std::str::FromStr;

/*
    Since the input does not have a known length, we interpret a '-' dash character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();

    let mut location = PathBuf::new();
    let mut files: Vec<(PathBuf, u64)> = Vec::new();
    let mut dirs: Vec<PathBuf> = vec![PathBuf::from("/")];
    for line in input.lines() {
        let line = line.expect("Read error");
        match line.as_ref() {
            "-" => {
                break;
            }
            "$ cd /" => {
                location.push("/");
            },
            "$ cd .." => {
                assert!(location.pop());
            },
            cd if line.starts_with("$ cd ") => {
                location.push(cd[5..].to_string());
            },
            "$ ls" => {
                // Since the input is well-formed, we can ignore this
                // We simply look at whether lines start with 'dir' or a 'number'
            },
            dir if line.starts_with("dir ") => {
                let mut sub_dir = location.clone();
                sub_dir.push(&dir[4..]);
                dirs.push(sub_dir);
            },
            _ => {
                let (size_str, _) = line.split_once(" ").expect("Failed split");
                let size = u64::from_str(size_str).expect("Parse failed");
                files.push((location.clone(), size));
            }
        }
    }

    let mut sum = 0;
    for dir in dirs {
        let total = files.iter().fold(0, |carry, elem| if elem.0.starts_with(&dir) { carry + elem.1 } else { carry });
        if total <= 100000 {
            sum += total;
        }
    }
    println!("{sum}");
}