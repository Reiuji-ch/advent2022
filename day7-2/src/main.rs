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

    let root_total = files.iter().fold(0, |carry, elem| carry + elem.1);
    let free_space = 70000000 - root_total;
    assert!(free_space < 30000000);
    let need_to_free = 30000000 - free_space;

    let mut best_option = u64::MAX;
    for dir in dirs {
        let total = files.iter().fold(0, |carry, elem| if elem.0.starts_with(&dir) { carry + elem.1 } else { carry });
        if total >= need_to_free && total < best_option {
            best_option = total;
        }
    }

    println!("{best_option}");
}