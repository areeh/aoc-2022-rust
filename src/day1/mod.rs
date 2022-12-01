use std::fs;

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day1/input.txt")
}

fn part1(calories: String) -> u32 {
    let (max_calories, _) = calories.lines().fold((0u32, 0u32), |(mut mx, mut acc), x| {
        if x.is_empty() {
            if acc > mx {
                mx = acc;
            }
            acc = 0;
        } else {
            acc += x.parse::<u32>().unwrap_or_else(|_| panic!("could not parse {} as digit", x));
        }
        (mx, acc)
    });
    max_calories
}

pub fn main() -> std::io::Result<()> {
    let input = input1()?;
    dbg!(part1(input));

    Ok(())
}

#[test]
fn example() {
    let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000".to_owned();
    assert_eq!(part1(input), 24000);
}

#[test]
fn task() {
    let input = input1().unwrap();
    assert_eq!(part1(input), 68292);
}