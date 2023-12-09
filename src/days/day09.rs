use std::io::{BufRead, Lines};

fn resolve<T>(lines: Lines<T>) -> (i64, i64)
where
    T: BufRead,
{
    lines.fold((0, 0), |(part1, part2), line| {
        let line = line.unwrap();

        let mut numbers = line
            .split_whitespace()
            .map(|digit| digit.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let mut start_accumulators = vec![];
        let mut p1 = 0;

        loop {
            p1 += numbers[numbers.len() - 1];
            start_accumulators.push(numbers[0]);

            numbers = numbers
                .iter()
                .zip(numbers.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect::<Vec<_>>();

            if numbers.iter().all(|&n| n == 0) {
                break;
            }
        }

        let p2 = start_accumulators.iter().rev().fold(0, |prev, s| s - prev);

        (part1 + p1, part2 + p2)
    })
}

#[test]
fn check() {
    const TEST: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (114, 2));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2023::Day::new(file!(), resolve_string) }
