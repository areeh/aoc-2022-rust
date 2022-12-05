use std::iter::once;

use crate::utils::read_input_to_string;

fn part1(calories: &str) -> u32 {
    let (max_calories, _) =
        calories
            .lines()
            .chain(once(""))
            .fold((0u32, 0u32), |(mut mx, mut acc), x| {
                if x.is_empty() {
                    if acc > mx {
                        mx = acc;
                    }
                    acc = 0;
                } else {
                    acc += x
                        .parse::<u32>()
                        .unwrap_or_else(|_| panic!("could not parse {} as digit", x));
                }
                (mx, acc)
            });
    max_calories
}

fn maybe_insert(v: u32, mxs: &mut [u32; 3]) {
    if v > mxs[2] {
        mxs[2] = v;
        if mxs[2] > mxs[1] {
            mxs.swap(1, 2);
            if mxs[1] > mxs[0] {
                mxs.swap(0, 1);
            }
        }
    }
}

fn part2(calories: &str) -> u32 {
    let (max_calories, _) =
        calories
            .lines()
            .chain(once(""))
            .fold(([0u32; 3], 0u32), |(mut mxs, mut acc), x| {
                if x.is_empty() {
                    maybe_insert(acc, &mut mxs);
                    acc = 0;
                } else {
                    acc += x
                        .parse::<u32>()
                        .unwrap_or_else(|_| panic!("could not parse {} as digit", x));
                }
                (mxs, acc)
            });
    max_calories.iter().sum()
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(1)?;
    dbg!(part1(input));
    dbg!(part2(input));

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

10000";
    assert_eq!(part1(input), 24000);
    assert_eq!(part2(input), 45000);
}

#[test]
fn task() {
    let input = &read_input_to_string(1).unwrap();
    assert_eq!(part1(input), 68292);
    assert_eq!(part2(input), 203203);
}
