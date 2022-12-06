extern crate test;

use std::{fs::File, io::Read};

#[cfg(test)]
use test::Bencher;

fn parts(window_size: usize) -> Option<usize> {
    // type u8 handles up to 256 occurences per byte
    // size to match range of u8
    let mut occurrences = [0u8; 256];
    let mut f = File::open("./src/day6/input.txt").unwrap();
    let mut buffer = [0; 4096];
    let mut n_duplicates = 0u8;
    f.read_exact(&mut buffer[..]).unwrap();

    // Track all information needed to check for return at each update
    for i in 0..4096 {
        let entering_byte = buffer[i] as usize;

        // going from not duplicated => duplicated can only occur here
        occurrences[entering_byte] += 1;
        if occurrences[entering_byte] == 2 {
            n_duplicates += 1;
        }

        // need to fill window before checking for return or evicting bytes
        if i >= window_size {
            let exiting_byte = buffer[i - window_size] as usize;

            // going from duplicated => not duplicated can only occur here
            occurrences[exiting_byte] -= 1;
            if occurrences[exiting_byte] == 1 {
                n_duplicates -= 1;
            }

            if n_duplicates == 0 {
                return Some(i + 1);
            }
        }
    }
    None
}

fn part1() -> usize {
    parts(4).unwrap()
}

fn part2() -> usize {
    parts(14).unwrap()
}

pub fn main() -> std::io::Result<()> {
    dbg!(part1());
    dbg!(part2());

    Ok(())
}

#[test]
fn example() {
    for (input, p1, p2) in [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, ()),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, ()),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 6, ()),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, ()),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, ()),
    ] {
        // assert_eq!(part1(input), p1);
        // assert_eq!(part2(input), ());
    }
}

#[test]
fn task() {
    assert_eq!(part1(), 1766);
    assert_eq!(part2(), 2383);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        parts(4);
        parts(14);
        parts(60);
    })
}
