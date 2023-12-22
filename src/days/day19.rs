use advent_2023::Paragrapher;
use std::collections::HashMap;
use std::io::{BufRead, Lines};

type Shape = [u64; 4];
type Range = [(u64, u64); 4];

fn combinations(range: &Range) -> u64 {
    range.iter().map(|(a, b)| b - a).product()
}

fn range_empty(range: &Range) -> bool {
    range.iter().all(|(a, b)| a == b)
}

type Interval = Option<(u64, u64)>;

fn interval_split_greater(a: u64, b: u64, value: u64) -> (Interval, Interval) {
    if value < a {
        (Some((a, b)), None)
    } else if value >= b {
        (None, Some((a, b)))
    } else {
        (Some((value + 1, b)), Some((a, value + 1)))
    }
}

fn interval_split_less(a: u64, b: u64, value: u64) -> (Interval, Interval) {
    if value <= a {
        (None, Some((a, b)))
    } else if value >= b {
        (Some((a, b)), None)
    } else {
        (Some((a, value)), Some((value, b)))
    }
}

#[test]
fn check_interval() {
    assert_eq!(interval_split_greater(10, 40, 5), (Some((10, 40)), None));
    assert_eq!(
        interval_split_greater(10, 40, 10),
        (Some((11, 40)), Some((10, 11)))
    );
    assert_eq!(
        interval_split_greater(10, 40, 20),
        (Some((21, 40)), Some((10, 21)))
    );
    assert_eq!(interval_split_greater(10, 40, 40), (None, Some((10, 40))));

    assert_eq!(interval_split_less(10, 40, 5), (None, Some((10, 40))));
    assert_eq!(interval_split_less(10, 40, 10), (None, Some((10, 40))));
    assert_eq!(
        interval_split_less(10, 40, 20),
        (Some((10, 20)), Some((20, 40)))
    );
    assert_eq!(interval_split_less(10, 40, 40), (Some((10, 40)), None));
}

#[derive(Debug)]
struct Condition {
    index: usize,
    value: u64,
    is_greater: bool,
}

impl Condition {
    fn new(condition: &[u8]) -> Self {
        let index = match condition[0] {
            b'x' => 0,
            b'm' => 1,
            b'a' => 2,
            b's' => 3,
            _ => panic!(),
        };
        let value = condition[2..]
            .iter()
            .fold(0, |v, c| 10 * v + (c - b'0') as u64);
        let is_greater = condition[1] == b'>';

        Condition {
            index,
            value,
            is_greater,
        }
    }

    fn is_ok(&self, shape: &Shape) -> bool {
        if self.is_greater {
            shape[self.index] > self.value
        } else {
            shape[self.index] < self.value
        }
    }

    fn split(&self, range: &Range) -> (Range, Range) {
        let mut ok = *range;
        let mut nok = *range;
        let (a, b);

        if self.is_greater {
            (a, b) = interval_split_greater(range[self.index].0, range[self.index].1, self.value);
        } else {
            (a, b) = interval_split_less(range[self.index].0, range[self.index].1, self.value);
        }

        if let Some(a) = a {
            ok[self.index] = a;
        }
        if let Some(b) = b {
            nok[self.index] = b;
        }

        (ok, nok)
    }
}

#[derive(Debug)]
struct Rule {
    conditions: Vec<(Condition, String)>,
    default: String,
}

impl Rule {
    fn new(default: &str, conditions: Vec<(Condition, String)>) -> Self {
        let default = default.to_string();
        Rule {
            conditions,
            default,
        }
    }

    fn destination(&self, shape: &Shape) -> &String {
        for (c, name) in self.conditions.iter() {
            if c.is_ok(shape) {
                return name;
            }
        }

        &self.default
    }

    fn destinations(&self, range: &Range) -> Vec<(&String, Range)> {
        let mut rs = vec![];
        let mut r = *range;

        for (c, name) in self.conditions.iter() {
            let (ok, nok) = c.split(&r);

            if !range_empty(&ok) {
                rs.push((name, ok));
            }

            if range_empty(&nok) {
                return rs;
            }

            r = nok;
        }

        rs.push((&self.default, r));

        rs
    }
}

fn is_accepted_shape(rules: &HashMap<String, Rule>, shape: &Shape) -> bool {
    let mut name = "in";

    loop {
        name = rules.get(name).unwrap().destination(shape);

        match name {
            "A" => return true,
            "R" => break,
            _ => {}
        }
    }
    false
}

fn part2(rules: &HashMap<String, Rule>) -> u64 {
    let starting = String::from("in");
    let mut ranges: Vec<(&String, Range)> = vec![(&starting, [(1, 4001); 4])];
    let mut count = 0;

    while let Some((name, range)) = ranges.pop() {
        for (n, r) in rules.get(name).unwrap().destinations(&range).into_iter() {
            match n.as_str() {
                "A" => count += combinations(&r),
                "R" => {}
                _ => ranges.push((n, r)),
            }
        }
    }

    count
}

fn resolve<T>(mut lines: Lines<T>) -> (u64, u64)
where
    T: BufRead,
{
    let mut para_iter = lines.split_paragraph(|l| l);

    let rules = para_iter
        .next()
        .unwrap()
        .iter()
        .map(|line| {
            let mut cs = vec![];
            let i = line.find('{').unwrap();
            let name = &line[0..i];

            for r in line[i + 1..line.len() - 1].split(',') {
                if let Some(i) = r.find(':') {
                    let condition = &r[0..i];
                    let destination = &r[i + 1..];

                    cs.push((
                        Condition::new(condition.as_bytes()),
                        destination.to_string(),
                    ));
                } else {
                    return (name.to_string(), Rule::new(r, cs));
                }
            }
            panic!();
        })
        .collect::<HashMap<_, _>>();

    let shapes = para_iter
        .next()
        .unwrap()
        .iter()
        .map(|line| {
            let mut s: Shape = [0; 4];

            for (idx, v) in line
                .split(|c: char| !c.is_ascii_digit())
                .filter(|s| !s.is_empty())
                .enumerate()
            {
                s[idx] = v.parse::<u64>().unwrap();
            }
            s
        })
        .collect::<Vec<_>>();

    let mut part1 = 0;

    for s in shapes {
        if is_accepted_shape(&rules, &s) {
            for v in s.iter() {
                part1 += v;
            }
        }
    }

    (part1, part2(&rules))
}

#[test]
fn check() {
    const TEST: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (19114, 167409079868000));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2023::Day::new(file!(), resolve_string) }
