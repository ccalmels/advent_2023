use std::io::{BufRead, Lines};

fn calibration_value_part1(line: &str) -> (u32, u32, usize, usize)
{
    let (first_digit, last_digit, line_end, line_start);

    if let Some(i) = line.find(|c: char|c.is_ascii_digit()) {
        line_end = i;
        first_digit = line.chars().nth(i).unwrap() as u32 - '0' as u32;
    } else {
        line_end = line.len();
        first_digit = 0;
    }

    if let Some(i) = line.rfind(|c: char|c.is_ascii_digit()) {
        line_start = i + 1;
        last_digit = line.chars().nth(i).unwrap() as u32 - '0' as u32;
    } else {
        line_start = 0;
        last_digit = 0;
    }

    (first_digit, last_digit, line_end, line_start)
}

fn calibration_value(line: &str) -> (u32, u32)
{
    static DIGIT_STRINGS: &[&str] =
        &[ "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" ];

    let (mut first_digit, mut last_digit, mut line_end, mut line_start) = calibration_value_part1(line);
    let part1 = first_digit * 10 + last_digit;

    for (idx, digit) in DIGIT_STRINGS.iter().enumerate() {
        if let Some(i) = line[0..line_end].find(digit) {
            line_end = i + digit.len() - 1;
            first_digit = idx as u32 + 1;
        }

        if let Some(i) = line[line_start..].rfind(digit) {
            line_start = i + line_start + 1;
            last_digit = idx as u32 + 1;
        }
    }

    (part1, first_digit * 10 + last_digit)
}

fn resolve<T>(lines: Lines<T>) -> (u32, u32)
where
    T: BufRead,
{
    lines.fold((0,0), |(part1, part2), line| {
        let line = line.unwrap();
        let (c1, c2) = calibration_value(&line);

        (part1 + c1, part2 + c2)
    })
}

#[test]
fn check() {
    const TEST1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    const TEST2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    use std::io::Cursor;

    let (part1, _) = resolve(Cursor::new(TEST1).lines());
    assert_eq!(part1, 142);
    let (_, part2) = resolve(Cursor::new(TEST2).lines());
    assert_eq!(part2, 281);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2023::Day::new(file!(), resolve_string) }
