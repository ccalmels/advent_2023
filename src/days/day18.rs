use std::io::{BufRead, Lines};

fn double_surface(p1: &(i64, i64), p2: &(i64, i64)) -> i64 {
    p2.0 * p1.1 - p2.1 * p1.0
}

#[test]
fn check_surface() {
    assert_eq!(double_surface(&(0, 2), &(3, 2)), 6);
    assert_eq!(double_surface(&(3, 2), &(0, 2)), -6);

    assert_eq!(double_surface(&(2, 3), &(6, 3)), 12);
    assert_eq!(double_surface(&(6, 3), &(2, 3)), -12);

    assert_eq!(double_surface(&(3, 0), &(3, -2)), 6);
    assert_eq!(double_surface(&(3, -2), &(3, 0)), -6);

    let square = [(2, 0), (6, 0), (6, -5), (2, -5), (2, 0)];
    let s: i64 = square
        .iter()
        .zip(square.iter().skip(1))
        .map(|(p1, p2)| double_surface(p1, p2))
        .sum();

    assert_eq!(s, 40);

    let square = [(2, 0), (7, 0), (7, -5), (2, -5), (2, 0)];
    let s: i64 = square
        .iter()
        .zip(square.iter().skip(1))
        .map(|(p1, p2)| double_surface(p1, p2))
        .sum();

    assert_eq!(s, 50);
}

fn color_to_order(hexa: &[u8]) -> ((i64, i64), i64) {
    let len = hexa.len();
    let mut v = 0;

    for &c in &hexa[2..len - 2] {
        v = 16 * v
            + match c {
                b'a'..=b'f' => (10 + c - b'a') as i64,
                b'0'..=b'9' => (c - b'0') as i64,
                _ => panic!(),
            };
    }

    (
        match &hexa[len - 2] {
            b'0' => (1, 0),
            b'1' => (0, 1),
            b'2' => (-1, 0),
            b'3' => (0, -1),
            _ => panic!(),
        },
        v,
    )
}

struct Part {
    digger: (i64, i64),
    double_surface: i64,
    perimeter: i64,
}

impl Part {
    fn new() -> Self {
        let digger = (0, 0);
        let double_surface = 0;
        let perimeter = 0;

        Part {
            digger,
            double_surface,
            perimeter,
        }
    }

    fn dig(&mut self, direction: (i64, i64), steps: i64) {
        let next = (
            self.digger.0 + steps * direction.0,
            self.digger.1 + steps * direction.1,
        );

        self.double_surface += double_surface(&self.digger, &next);
        self.perimeter += steps;
        self.digger = next;
    }

    fn result(&self) -> i64 {
        1 + (self.double_surface.abs() + self.perimeter) / 2
    }
}

fn resolve<T>(lines: Lines<T>) -> (i64, i64)
where
    T: BufRead,
{
    let mut part1 = Part::new();
    let mut part2 = Part::new();

    for line in lines {
        let line = line.unwrap();
        let v = line.split_whitespace().collect::<Vec<_>>();
        let direction = match v[0] {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => panic!(),
        };
        let steps = v[1].parse::<i64>().unwrap();

        part1.dig(direction, steps);

        let (direction, steps) = color_to_order(v[2].as_bytes());

        part2.dig(direction, steps);
    }

    (part1.result(), part2.result())
}

#[test]
fn check() {
    const TEST: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (62, 952408144115));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2023::Day::new(file!(), resolve_string) }
