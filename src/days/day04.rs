use regex::Regex;
use std::collections::VecDeque;
use std::io::{BufRead, Lines};

fn resolve<T>(lines: Lines<T>) -> (u32, usize)
where
    T: BufRead,
{
    let re = Regex::new(r"^Card\s+(\d+): ([\d ]+) \| ([\d ]+)$").unwrap();
    let mut copies = VecDeque::new();
    let mut part2 = 0;

    let part1 = lines
        .filter_map(|line| {
            let line = line.unwrap();

            let m = re.captures(&line).unwrap();
            let winning_numbers = m
                .get(2)
                .unwrap()
                .as_str()
                .split_whitespace()
                .collect::<Vec<_>>();
            let own_numbers = m.get(3).unwrap().as_str().split_whitespace();

            let n = own_numbers
                .filter(|&own| winning_numbers.contains(&own))
                .count();

            let copies_1 = 1 + copies.pop_front().unwrap_or(0);

            part2 += copies_1;

            if copies.len() < n {
                copies.resize(n, 0);
            }

            for copy in copies.iter_mut().take(n) {
                *copy += copies_1;
            }

            if n == 0 {
                None
            } else {
                Some(2u32.pow(n as u32 - 1))
            }
        })
        .sum();

    (part1, part2)
}

#[test]
fn check() {
    const TEST: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    use std::io::Cursor;

    let (part1, part2) = resolve(Cursor::new(TEST).lines());

    assert_eq!(part1, 13);
    assert_eq!(part2, 30);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2023::Day::new(file!(), resolve_string) }
