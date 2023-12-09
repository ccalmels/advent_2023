use std::collections::HashMap;
use std::io::{BufRead, Lines};

fn gcd(a: usize, b: usize) -> usize {
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

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[test]
fn check_lcm() {
    assert_eq!(lcm(14893, 19951), 1057403);
    assert_eq!(lcm(1057403, 22199), 83534837);
    assert_eq!(lcm(1, 23456), 23456);
}

fn compute(path: &[(usize, usize)], start: usize, ends: &[usize], sequence: &[bool]) -> usize {
    let mut c = start;
    let mut counter = 1;
    let mut sequence_index = 0;

    loop {
        c = if sequence[sequence_index] {
            path[c].0
        } else {
            path[c].1
        };

        if ends.contains(&c) {
            return counter;
        }

        counter += 1;
        sequence_index = (sequence_index + 1) % sequence.len();
    }
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
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
    let mut starting_z = vec![];
    let mut ending_z = vec![];
    let (mut index_aaa, mut index_zzz) = (None, None);

    lines.next();

    let mut graph = HashMap::new();

    for line in lines {
        let line = line.unwrap();

        let name = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];

        if &line[2..3] == "A" {
            starting_z.push(graph.len());

            if &line[0..2] == "AA" {
                index_aaa = Some(graph.len());
            }
        }
        if &line[2..3] == "Z" {
            ending_z.push(graph.len());

            if &line[0..2] == "ZZ" {
                index_zzz = Some(graph.len());
            }
        }

        graph.insert(
            name.to_string(),
            (graph.len(), left.to_string(), right.to_string()),
        );
    }

    let mut path = vec![(0, 0); graph.len()];

    for (index, l, r) in graph.values() {
        path[*index] = (graph.get(l).unwrap().0, graph.get(r).unwrap().0);
    }

    let part1;

    if let Some(index_aaa) = index_aaa {
        let index_zzz = index_zzz.unwrap();

        part1 = compute(&path, index_aaa, &[index_zzz], &sequence);
    } else {
        part1 = 0;
    }

    let part2 = starting_z
        .iter()
        .map(|&s| compute(&path, s, &ending_z, &sequence))
        .fold(1, lcm);

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
