use std::cmp::{Ordering, Ordering::Equal};
use std::io::{BufRead, Lines};

fn card_value(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '2'..='9' => c as u32 - '0' as u32,
        _ => panic!(""),
    }
}

fn get_type(cards: &Vec<(u32, u32)>) -> u32 {
    match cards.len() {
        1 => 7,
        2 => {
            if (cards[0].0 == 1) || (cards[0].0 == 4) {
                6
            } else {
                5
            }
        }
        3 => {
            if cards.iter().any(|&(c, _)| c == 3) {
                4
            } else {
                3
            }
        }
        4 => 2,
        5 => 1,
        _ => panic!(""),
    }
}

#[derive(Debug, Eq, Copy, Clone)]
struct HandValue {
    ctype: u32,
    value: u32,
}

impl HandValue {
    fn part1(hand: &str) -> Self {
        let mut cards = vec![];
        let mut value = 0;

        for c in hand.chars() {
            let v = card_value(c);
            let found = cards.iter_mut().find(|(_, value)| *value == v);
            if let Some(found) = found {
                found.0 += 1;
            } else {
                cards.push((1, v));
            }
            value = value * 16 + v;
        }

        let ctype = get_type(&cards);

        HandValue { ctype, value }
    }

    fn part2(hand: &str) -> Self {
        let mut cards = vec![];
        let mut value = 0;
        let mut j_count = 0;

        for c in hand.chars() {
            let v = card_value(c);

            if v == 11 {
                value = value * 16 + 1;
                j_count += 1;
            } else {
                let found = cards.iter_mut().find(|(_, value)| *value == v);
                if let Some(found) = found {
                    found.0 += 1;
                } else {
                    cards.push((1, v));
                }
                value = value * 16 + v;
            }
        }

        if j_count > 0 {
            let max_card = cards.iter_mut().max_by(|a, b| {
                let ret = a.0.cmp(&b.0);

                if ret == Equal {
                    a.1.cmp(&b.1)
                } else {
                    ret
                }
            });
            if let Some(max_card) = max_card {
                max_card.0 += j_count;
            } else {
                cards.push((5, 14));
            }
        }

        let ctype = get_type(&cards);

        HandValue { ctype, value }
    }
}

impl PartialOrd for HandValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandValue {
    fn cmp(&self, other: &Self) -> Ordering {
        let ret = self.ctype.cmp(&other.ctype);

        if ret == Equal {
            self.value.cmp(&other.value)
        } else {
            ret
        }
    }
}

impl PartialEq for HandValue {
    fn eq(&self, other: &Self) -> bool {
        if self.ctype.eq(&other.ctype) {
            self.value.eq(&other.value)
        } else {
            false
        }
    }
}

#[test]
fn check_handvalue() {
    assert_eq!(
        HandValue::part1("AAAAA"),
        HandValue {
            ctype: 7,
            value: 0xeeeee
        }
    );
    assert_eq!(
        HandValue::part1("22222"),
        HandValue {
            ctype: 7,
            value: 0x22222
        }
    );
    assert_eq!(
        HandValue::part1("TTT32"),
        HandValue {
            ctype: 4,
            value: 0xaaa32
        }
    );
    assert_eq!(
        HandValue::part1("33AA2"),
        HandValue {
            ctype: 3,
            value: 0x33ee2
        }
    );
    assert_eq!(
        HandValue::part1("23456"),
        HandValue {
            ctype: 1,
            value: 0x23456
        }
    );
    assert_eq!(
        HandValue::part1("JJJJJ"),
        HandValue {
            ctype: 7,
            value: 0xbbbbb
        }
    );

    assert!(HandValue::part1("22223") == HandValue::part1("22223"));
    assert!(HandValue::part1("2222A") > HandValue::part1("22223"));
    assert!(HandValue::part1("2222A") > HandValue::part1("AAA22"));
    assert!(HandValue::part1("AAA23") < HandValue::part1("33322"));
    assert!(HandValue::part1("AAA23") > HandValue::part1("33522"));
    assert!(HandValue::part1("AAA23") > HandValue::part1("3352A"));
    assert!(HandValue::part1("23456") < HandValue::part1("3352A"));
    assert!(HandValue::part1("KKK23") < HandValue::part1("AAA23"));
    assert!(HandValue::part1("KK223") < HandValue::part1("AA553"));
    assert!(HandValue::part1("KK223") > HandValue::part1("55AA3"));

    assert_eq!(
        HandValue::part2("33AAJ"),
        HandValue {
            ctype: 5,
            value: 0x33ee1
        }
    );
    assert_eq!(
        HandValue::part2("TJJ32"),
        HandValue {
            ctype: 4,
            value: 0xa1132
        }
    );
    assert_eq!(
        HandValue::part2("JJJJJ"),
        HandValue {
            ctype: 7,
            value: 0x11111
        }
    );
}

#[derive(Debug)]
struct Hand {
    bid: u32,
    part1: HandValue,
    part2: HandValue,
}

fn resolve<T>(lines: Lines<T>) -> (u32, u32)
where
    T: BufRead,
{
    let mut hands = vec![];

    for line in lines {
        let line = line.unwrap();
        let words = line.split_whitespace().collect::<Vec<_>>();

        hands.push(Hand {
            bid: words[1].parse::<u32>().unwrap(),
            part1: HandValue::part1(words[0]),
            part2: HandValue::part2(words[0]),
        });
    }

    hands.sort_unstable_by_key(|hand| hand.part1);

    let part1 = hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.bid * (idx as u32 + 1))
        .sum();

    hands.sort_unstable_by_key(|hand| hand.part2);

    let part2 = hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.bid * (idx as u32 + 1))
        .sum();

    (part1, part2)
}

#[test]
fn check() {
    const TEST: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (6440, 5905));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2023::Day::new(file!(), resolve_string) }
