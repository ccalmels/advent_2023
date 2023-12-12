use std::io::{BufRead, Lines};

fn calibration_value(line: &str) -> (u32, u32) {
    static DIGIT_STRINGS: &[&str] = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let (mut first_digit, mut last_digit) = (0, 0);
    let (mut first_digit_s, mut last_digit_s) = (None, None);

    for (i, c) in line.chars().enumerate() {
        if c.is_ascii_digit() {
            first_digit = c as u32 - '0' as u32;
            break;
        }

        if first_digit_s.is_none() {
            for (value_1, digit) in DIGIT_STRINGS.iter().enumerate() {
                if line[i..].starts_with(digit) {
                    first_digit_s = Some(value_1 as u32 + 1);
                }
            }
        }
    }

    for (i, c) in line.chars().rev().enumerate() {
        if c.is_ascii_digit() {
            last_digit = c as u32 - '0' as u32;
            break;
        }

        if last_digit_s.is_none() {
            for (value_1, digit) in DIGIT_STRINGS.iter().enumerate() {
                if line[line.len() - i - 1..].starts_with(digit) {
                    last_digit_s = Some(value_1 as u32 + 1);
                }
            }
        }
    }

    (
        first_digit * 10 + last_digit,
        first_digit_s.unwrap_or(first_digit) * 10 + last_digit_s.unwrap_or(last_digit),
    )
}

fn resolve<T>(lines: Lines<T>) -> (u32, u32)
where
    T: BufRead,
{
    lines.fold((0, 0), |(part1, part2), line| {
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

    assert_eq!(resolve(Cursor::new(TEST1).lines()).0, 142);
    assert_eq!(resolve(Cursor::new(TEST2).lines()).1, 281);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2023::Day::new(file!(), resolve_string) }
