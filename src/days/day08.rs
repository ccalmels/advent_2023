use std::collections::HashMap;
use std::io::{BufRead, Lines};

fn gcd(a: u64, b: u64) -> u64 {
    let (mut max, mut min) = if a > b { (a, b) } else { (b, a) };

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

#[test]
fn check_lcm() {
    assert_eq!(lcm(14893, 19951), 1057403);
    assert_eq!(lcm(1057403, 22199), 83534837);
    assert_eq!(lcm(1, 23456), 23456);
}

fn resolve<T>(lines: Lines<T>) -> (u64, u64)
where
    T: BufRead,
{
    let mut lines = lines;
    let sequence = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => true,
            'R' => false,
            _ => panic!(""),
        })
        .collect::<Vec<_>>();
    let mut currents = vec![];
    let mut ends = vec![];
    let (mut index_current, mut index_zzz) = (None, None);

    lines.next();

    let mut graph = HashMap::new();

    for line in lines {
        let line = line.unwrap();

        let name = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];

        if &line[2..3] == "A" {
            currents.push(graph.len());

            if &line[0..2] == "AA" {
                index_current = Some(graph.len());
            }
        }
        if &line[2..3] == "Z" {
            ends.push(graph.len());

            if &line[0..2] == "ZZ" {
                index_zzz = Some(graph.len());
            }
        }

        graph.insert(
            name.to_string(),
            (graph.len(), left.to_string(), right.to_string()),
        );
    }

    let mut left = vec![0; graph.len()];
    let mut right = vec![0; graph.len()];

    for (index, l, r) in graph.values() {
        let l = graph.get(l).unwrap();
        let r = graph.get(r).unwrap();

        left[*index] = l.0;
        right[*index] = r.0;
    }

    let mut sequence_index = 0;
    let mut counter = 0;

    if let Some(mut index_current) = index_current {
        let index_zzz = index_zzz.unwrap();

        while index_current != index_zzz {
            counter += 1;

            index_current = if sequence[sequence_index] {
                left[index_current]
            } else {
                right[index_current]
            };

            sequence_index = (sequence_index + 1) % sequence.len();
        }
    }

    let part1 = counter;

    sequence_index = 0;
    counter = 0;
    let mut results = vec![];

    while !currents.is_empty() {
        counter += 1;

        currents = currents
            .into_iter()
            .filter_map(|c| {
                let c = if sequence[sequence_index] {
                    left[c]
                } else {
                    right[c]
                };

                if ends.contains(&c) {
                    results.push(counter);
                    None
                } else {
                    Some(c)
                }
            })
            .collect::<Vec<_>>();

        sequence_index = (sequence_index + 1) % sequence.len();
    }

    let part2 = results.into_iter().fold(1, |acc, num| lcm(acc, num));

    (part1, part2)
}

#[test]
fn check() {
    const TEST1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    const TEST2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    const TEST3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST1).lines()), (2, 2));
    assert_eq!(resolve(Cursor::new(TEST2).lines()), (6, 6));
    assert_eq!(resolve(Cursor::new(TEST3).lines()), (0, 6));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2023::Day::new(file!(), resolve_string) }
