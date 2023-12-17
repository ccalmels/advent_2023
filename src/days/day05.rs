use advent_2023::Paragrapher;
use std::cmp::Ordering;
use std::io::{BufRead, Lines};

#[derive(Debug, Eq)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn new(start: i64, length: i64) -> Self {
        let end = start + length;

        Range { start, end }
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.start.eq(&other.start)
    }
}

#[derive(Debug, Eq)]
struct Translation {
    range: Range,
    value: i64,
}

impl Translation {
    fn new(destination: i64, source: i64, size: i64) -> Self {
        let value = destination - source;
        let range = Range::new(source, size);

        Translation { range, value }
    }
}

impl PartialOrd for Translation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Translation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.range.cmp(&other.range)
    }
}

impl PartialEq for Translation {
    fn eq(&self, other: &Self) -> bool {
        self.range.eq(&other.range)
    }
}

#[derive(Debug)]
struct Maps {
    translations: Vec<Translation>,
}

impl Maps {
    fn new() -> Self {
        Maps {
            translations: vec![],
        }
    }

    fn found(&self, value: i64) -> Option<&Translation> {
        self.translations
            .iter()
            .find(|t| (value >= t.range.start) && (value < t.range.end))
    }

    fn add_sorted(&mut self, destination: i64, source: i64, size: i64) {
        let t = Translation::new(destination, source, size);
        let pos = self.translations.binary_search(&t).unwrap_or_else(|e| e);

        self.translations.insert(pos, t);
    }

    fn part1(&self, seeds: &mut [i64]) {
        for s in seeds {
            let found = self.found(*s);

            if let Some(t) = found {
                *s += t.value;
            }
        }
    }

    fn part2_range(&self, r: &Range) -> Vec<Range> {
        let (mut start, mut end) = (r.start, r.end);
        let mut ranges = vec![];

        for t in self.translations.iter() {
            if end < t.range.start {
                break;
            }

            if start > t.range.end {
                continue;
            }

            if start < t.range.start {
                ranges.push(Range {
                    start,
                    end: t.range.start,
                });

                start = t.range.start;
            }

            if end < t.range.end {
                start += t.value;
                end += t.value;
                break;
            } else {
                ranges.push(Range {
                    start: start + t.value,
                    end: t.range.end + t.value,
                });

                start = t.range.end;
            }
        }

        ranges.push(Range { start, end });
        ranges
    }

    fn part2(&self, ranges: &[Range]) -> Vec<Range> {
        let mut ret = vec![];

        for r in ranges {
            for result in self.part2_range(r) {
                ret.push(result);
            }
        }

        ret
    }
}

#[test]
fn check_range() {
    let mut maps: Maps = Maps::new();

    maps.add_sorted(50, 98, 2);
    maps.add_sorted(52, 50, 48);

    let range = Range::new(3, 2);
    assert_eq!(maps.part2_range(&range), [Range { start: 3, end: 5 }]);

    let range = Range::new(55, 5);
    assert_eq!(maps.part2_range(&range), [Range { start: 57, end: 62 }]);

    let range = Range::new(45, 10);
    assert_eq!(
        maps.part2_range(&range),
        [Range { start: 45, end: 50 }, Range { start: 52, end: 57 }]
    );

    let range = Range::new(99, 2);
    assert_eq!(
        maps.part2_range(&range),
        [
            Range { start: 51, end: 52 },
            Range {
                start: 100,
                end: 101
            }
        ]
    );

    let range = Range::new(45, 150);
    assert_eq!(
        maps.part2_range(&range),
        [
            Range { start: 45, end: 50 },
            Range {
                start: 52,
                end: 100
            },
            Range { start: 50, end: 52 },
            Range {
                start: 100,
                end: 205
            },
        ]
    );
}

fn resolve<T>(mut lines: Lines<T>) -> (i64, i64)
where
    T: BufRead,
{
    let mut para_iter = lines.split_paragraph();

    let mut seeds = para_iter.next().unwrap()[0]
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse::<i64>().ok())
        .collect::<Vec<_>>();
    let mut seeds_ranges = seeds
        .iter()
        .step_by(2)
        .zip(seeds.iter().skip(1).step_by(2))
        .map(|(&start, &length)| Range::new(start, length))
        .collect::<Vec<_>>();

    for p in para_iter {
        let mut maps = Maps::new();

        for n in p.into_iter().skip(1) {
            let numbers = n
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            assert_eq!(numbers.len(), 3);
            maps.add_sorted(numbers[0], numbers[1], numbers[2]);
        }

        maps.part1(&mut seeds);
        seeds_ranges = maps.part2(&seeds_ranges);
    }

    seeds_ranges.sort_unstable();

    (*seeds.iter().min().unwrap(), seeds_ranges[0].start)
}

#[test]
fn check() {
    const TEST: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (35, 46));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2023::Day::new(file!(), resolve_string) }
