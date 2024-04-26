use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::{BufRead, Lines};

#[derive(Debug, PartialEq)]
enum SecondDegreeResult {
    None,
    One(f64),
    Two(f64, f64),
}

fn second_degree(a: i64, b: i64, c: i64) -> SecondDegreeResult {
    let delta = b * b - 4 * a * c;

    match delta.cmp(&0) {
        Less => SecondDegreeResult::None,
        Equal => SecondDegreeResult::One(-(b as f64) / (2 * a) as f64),
        Greater => SecondDegreeResult::Two(
            -((b as f64 - (delta as f64).sqrt()) / (2 * a) as f64),
            -((b as f64 + (delta as f64).sqrt()) / (2 * a) as f64),
        ),
    }
}

#[test]
fn check_second_degree() {
    // x^2 + 1 = 0
    assert_eq!(second_degree(1, 0, 1), SecondDegreeResult::None);

    // (x + 1)^2 = 0
    assert_eq!(second_degree(1, 2, 1), SecondDegreeResult::One(-1.0));

    // x^2 - 1 = 0
    assert_eq!(second_degree(1, 0, -1), SecondDegreeResult::Two(1.0, -1.0));
}

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

fn winners(race: &Race) -> i64 {
    let a = -1;
    let b = race.time;
    let c = -race.distance;
    let roots = second_degree(a, b, c);

    match roots {
        SecondDegreeResult::None => 0,
        SecondDegreeResult::One(_) => 1,
        SecondDegreeResult::Two(f1, f2) => (f2.ceil() - f1.floor() - 1.0) as i64,
    }
}

fn resolve<T>(lines: Lines<T>) -> (i64, i64)
where
    T: BufRead,
{
    let mut lines = lines;
    let (mut total_time, mut total_distance) = (String::new(), String::new());
    let mut races = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|time| {
            total_time.push_str(time);

            Race {
                time: time.parse::<i64>().unwrap(),
                distance: 0,
            }
        })
        .collect::<Vec<_>>();

    lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .enumerate()
        .for_each(|(idx, distance)| {
            total_distance.push_str(distance);

            races[idx].distance = distance.parse::<i64>().unwrap();
        });

    let total_race = Race {
        time: total_time.parse::<i64>().unwrap(),
        distance: total_distance.parse::<i64>().unwrap(),
    };

    (races.iter().map(winners).product(), winners(&total_race))
}

#[test]
fn check() {
    const TEST: &str = "Time:      7  15   30
Distance:  9  40  200";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (288, 71503));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2023::Day::new(file!(), resolve_string) }
