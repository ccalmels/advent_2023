use std::io::{BufRead, Lines};

fn hash(s: &[u8]) -> usize {
    let mut value: usize = 0;

    for c in s {
        value += *c as usize;
        value *= 17;
        value %= 256;
    }
    value
}

#[test]
fn check_hash() {
    assert_eq!(hash("HASH".as_bytes()), 52);
}

fn resolve<T>(mut lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let line = lines.next().unwrap().unwrap();
    let mut part1 = 0;
    let mut boxes: Vec<Vec<(&[u8], u8)>> = vec![vec![]; 256];

    for s in line.split(',') {
        let s = s.as_bytes();

        part1 += hash(s);

        let equal = s.iter().position(|&x| x == b'=');

        if let Some(equal) = equal {
            let h = hash(&s[0..equal]);
            let l = boxes[h]
                .iter_mut()
                .find(|(label, _)| *label == &s[0..equal]);
            let v = s[equal + 1] - b'0';
            if let Some(l) = l {
                l.1 = v;
            } else {
                boxes[h].push((&s[0..equal], v));
            }
        } else {
            let h = hash(&s[0..s.len() - 1]);
            let l = boxes[h]
                .iter()
                .position(|(label, _)| *label == &s[0..s.len() - 1]);
            if let Some(l) = l {
                boxes[h].remove(l);
            }
        }
    }

    let mut part2 = 0;

    for (i, b) in boxes.iter().enumerate() {
        for (j, l) in b.iter().enumerate() {
            part2 += (i + 1) * (j + 1) * l.1 as usize;
        }
    }

    (part1, part2)
}

#[test]
fn check() {
    const TEST: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (1320, 145));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2023::Day::new(file!(), resolve_string) }
