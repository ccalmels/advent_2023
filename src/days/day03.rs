use regex::Regex;
use std::io::{BufRead, Lines};

#[derive(Debug)]
struct Number {
    value: u32,
    a: (i32, i32),
    b: (i32, i32),
}

impl Number {
    fn new(value: u32, start: i32, end: i32, y: i32) -> Self {
        let a = (start - 1, y - 1);
        let b = (end, y + 1);

        Number { value, a, b }
    }

    fn contains(&self, (x, y): (i32, i32)) -> bool {
        !(x < self.a.0 || x > self.b.0 || y < self.a.1 || y > self.b.1)
    }

    fn is_active(&self, points: &[(i32, i32)]) -> bool {
        points.iter().any(|&p| self.contains(p))
    }
}

#[test]
fn check_number() {
    let n = Number::new(123, 2, 5, 3);

    assert!(!n.contains((2, 1)));
    assert!(n.contains((2, 2)));
    assert!(n.contains((2, 3)));
    assert!(n.contains((2, 4)));
    assert!(!n.contains((2, 5)));

    assert!(!n.contains((0, 3)));
    assert!(n.contains((2, 3)));
    assert!(n.contains((5, 3)));
    assert!(!n.contains((6, 3)));
}

fn resolve<T>(lines: Lines<T>) -> (u32, u32)
where
    T: BufRead,
{
    let re = Regex::new(r"(?<number>\d+)|(?<symbol>[^\.])").unwrap();
    let mut numbers = vec![];
    let mut symbols = vec![];
    let mut asterix = vec![];

    for (y, line) in lines.enumerate() {
        let line = line.unwrap();

        for capture in re.captures_iter(&line) {
            if let Some(m) = capture.name("number") {
                let n = m.as_str().parse::<u32>().unwrap();

                numbers.push(Number::new(n, m.start() as i32, m.end() as i32, y as i32));
            } else if let Some(m) = capture.name("symbol") {
                symbols.push((m.start() as i32, y as i32));

                if m.as_str() == "*" {
                    asterix.push((m.start() as i32, y as i32));
                }
            } else {
                panic!();
            }
        }
    }

    let part1 = numbers
        .iter()
        .filter_map(|n| {
            if n.is_active(&symbols) {
                Some(n.value)
            } else {
                None
            }
        })
        .sum();

    let part2 = asterix
        .iter()
        .filter_map(|&a| {
            let values = numbers
                .iter()
                .filter_map(|n| {
                    if n.contains(a) {
                        Some(n.value)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if values.len() > 1 {
                Some(values.iter().product::<u32>())
            } else {
                None
            }
        })
        .sum();

    (part1, part2)
}

#[test]
fn check() {
    const TEST: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    use std::io::Cursor;

    let (part1, part2) = resolve(Cursor::new(TEST).lines());

    assert_eq!(part1, 4361);
    assert_eq!(part2, 467835);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2023::Day::new(file!(), resolve_string) }
