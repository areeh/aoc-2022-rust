extern crate test;

use std::collections::HashMap;

#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

#[derive(Debug, Clone)]
enum FileItem {
    Dir(String),
    File(usize),
}

type FS = HashMap<String, (Option<String>, Vec<FileItem>)>;

fn get_parent(f: Option<String>) -> Option<String> {
    if let Some(s) = f {
        s.rsplit_once('/').map(|(parent, _)| parent.into())
    } else {
        None
    }
}

fn make_child(f: Option<String>, next: &str) -> String {
    if let Some(s) = f {
        if s == "/" {
            format!("{}{}", s, next)
        } else {
            format!("{}/{}", s, next)
        }
    } else {
        next.into()
    }
}

fn calc_sizes(dir: &FileItem, dir_sizes: &mut Vec<usize>, fs: &FS) -> usize {
    match dir {
        FileItem::File(size) => *size,
        FileItem::Dir(dir) => {
            let (_, children) = &fs[dir];
            let size = children.iter().map(|v| calc_sizes(v, dir_sizes, fs)).sum();
            dir_sizes.push(size);
            size
        }
    }
}

fn build_fs(input: &str) -> FS {
    // TODO: Make this not horrible
    let mut line_iter = input.lines().peekable();
    let mut current_dir: Option<String> = None;
    let mut fs: FS = HashMap::new();
    let mut has_dirs = true;
    while has_dirs {
        current_dir = if let Some(line) = line_iter.next() {
            let mut word_iter = line.split_whitespace();
            match word_iter.next().unwrap() {
                "$" => {
                    match word_iter.next().unwrap() {
                        "cd" => {
                            match word_iter.next().unwrap() {
                                ".." => get_parent(current_dir),
                                c => {
                                    line_iter.advance_by(1).unwrap();
                                    let mut children: Vec<FileItem> = Vec::new();
                                    let next_dir = make_child(current_dir.clone(), c);

                                    while line_iter.peek().is_some()
                                        && !line_iter.peek().unwrap().starts_with('$')
                                    {
                                        let mut word_iter =
                                            line_iter.next().unwrap().split_whitespace();
                                        match word_iter.next().unwrap() {
                                    "dir" => {
                                        let child = make_child(Some(next_dir.clone()), word_iter.next().unwrap());
                                        children.push(FileItem::Dir(child));
                                    }
                                    digits => children.push(FileItem::File(
                                        digits.parse().unwrap_or_else(|_| {
                                            panic!("could not parse {} as digit, after that is {}", digits, word_iter.next().unwrap())
                                        }),
                                    )),
                                }
                                    }
                                    fs.insert(next_dir.clone(), (current_dir, children.clone()));
                                    Some(next_dir)
                                }
                            }
                        }
                        c => panic!("Unrecognized command {c}"),
                    }
                }
                c => panic!("Expected to find a command, got {c}"),
            }
        } else {
            None
        };
        if current_dir.is_none() {
            has_dirs = false
        }
    }
    fs
}

fn get_dir_sizes(input: &str) -> Vec<usize> {
    let fs = build_fs(input);
    let mut dir_sizes = Vec::new();
    calc_sizes(&FileItem::Dir("/".into()), &mut dir_sizes, &fs);
    dir_sizes
}

fn part1(input: &str) -> usize {
    let dir_sizes = get_dir_sizes(input);
    dir_sizes.iter().filter(|size| **size <= 100_000).sum()
}

fn part2(input: &str) -> usize {
    const TOTAL_SPACE: usize = 70_000_000;
    const WANTED_SPACE: usize = 30_000_000;

    let mut dir_sizes = get_dir_sizes(input);
    dir_sizes.sort();
    let used_space = &dir_sizes.last().unwrap();

    *dir_sizes
        .iter()
        .find(|v| **v > (WANTED_SPACE - (TOTAL_SPACE - **used_space)))
        .unwrap()
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(7)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example() {
    let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    assert_eq!(part1(input), 95437);
    assert_eq!(part2(input), 24933642);
}

#[test]
fn task() {
    let input = &read_input_to_string(7).unwrap();
    assert_eq!(part1(input), 1844187);
    assert_eq!(part2(input), 4978279);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(7).unwrap();
        part1(input);
        part2(input);
    })
}
