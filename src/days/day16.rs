use std::io::{BufRead, Lines};

#[derive(Debug)]
struct Beam {
    position: (i32, i32),
    direction: (i32, i32),
}

impl Beam {
    fn new(position: (i32, i32), direction: (i32, i32)) -> Self {
        Beam {
            position,
            direction,
        }
    }

    fn next(&self) -> Self {
        let position = (
            self.position.0 + self.direction.0,
            self.position.1 + self.direction.1,
        );
        Beam {
            position,
            direction: self.direction,
        }
    }

    fn left(&self) -> Self {
        let direction = (self.direction.1, -self.direction.0);

        Beam {
            position: self.position,
            direction,
        }
    }

    fn right(&self) -> Self {
        let direction = (-self.direction.1, self.direction.0);

        Beam {
            position: self.position,
            direction,
        }
    }

    fn split(&self) -> (Self, Self) {
        (self.left(), self.right())
    }
}

#[test]
fn check_rotation() {
    let mut b = Beam::new((0, 0), (0, -1));

    b = b.left();
    assert_eq!(b.direction, (-1, 0));

    b = b.left();
    assert_eq!(b.direction, (0, 1));

    b = b.left();
    assert_eq!(b.direction, (1, 0));

    b = b.left();
    assert_eq!(b.direction, (0, -1));

    b = b.right();
    assert_eq!(b.direction, (1, 0));

    b = b.right();
    assert_eq!(b.direction, (0, 1));

    b = b.right();
    assert_eq!(b.direction, (-1, 0));

    b = b.right();
    assert_eq!(b.direction, (0, -1));
}

fn energize(contraption: &[Vec<char>], starting: Beam) -> usize {
    let mut stack = vec![];
    let w = contraption[0].len();
    let h = contraption.len();
    let mut energized = vec![vec![false; w]; h];

    energized[starting.position.1 as usize][starting.position.0 as usize] = true;

    stack.push(starting);

    while let Some(beam) = stack.pop() {
        let beam = beam.next();

        if beam.position.0 < 0 || beam.position.1 < 0 {
            // we are outside the contraption
            continue;
        }

        let (x, y) = (beam.position.0 as usize, beam.position.1 as usize);

        if x >= w || y >= h {
            // we are outside the contraption
            continue;
        }

        match (contraption[y][x], beam.direction) {
            // do forward
            ('.', _) => stack.push(beam),
            ('|', (0, _)) => stack.push(beam),
            ('-', (_, 0)) => stack.push(beam),
            // turn
            ('/', (0, _)) => stack.push(beam.right()),
            ('/', (_, 0)) => stack.push(beam.left()),
            ('\\', (0, _)) => stack.push(beam.left()),
            ('\\', (_, 0)) => stack.push(beam.right()),
            // divide
            ('|', (_, 0)) => {
                if !energized[y][x] {
                    let (l, r) = beam.split();
                    stack.push(l);
                    stack.push(r);
                }
            }
            ('-', (0, _)) => {
                if !energized[y][x] {
                    let (l, r) = beam.split();
                    stack.push(l);
                    stack.push(r);
                }
            }
            _ => panic!(),
        }

        energized[y][x] = true;
    }

    energized.into_iter().flatten().filter(|&e| e).count()
}

fn part2(contraption: &[Vec<char>]) -> usize {
    let w = contraption[0].len();
    let h = contraption.len();
    let mut max_energy = 0;

    for y in 0..h {
        let b_right = Beam::new((0, y as i32), (1, 0));

        max_energy = max_energy.max(energize(contraption, b_right));

        let b_left = Beam::new((w as i32 - 1, y as i32), (-1, 0));

        max_energy = max_energy.max(energize(contraption, b_left));
    }

    for x in 0..w {
        let b_down = Beam::new((x as i32, 0), (0, 1));

        max_energy = max_energy.max(energize(contraption, b_down));

        let b_up = Beam::new((x as i32, h as i32 - 1), (0, -1));

        max_energy = max_energy.max(energize(contraption, b_up));
    }

    max_energy
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let contraption = lines
        .map(|line| {
            let line = line.unwrap();

            line.chars().collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (
        energize(&contraption, Beam::new((0, 0), (1, 0))),
        part2(&contraption),
    )
}

#[test]
fn check() {
    const TEST: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (46, 51));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2023::Day::new(file!(), resolve_string) }
