use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{BufRead, Lines};

#[derive(Debug, Eq, PartialEq)]
struct Crucible {
    position: (i32, i32),
    direction: (i32, i32),
    heat: u32,
}

impl Ord for Crucible {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat.cmp(&self.heat)
    }
}

impl PartialOrd for Crucible {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Crucible {
    fn new(position: (i32, i32), direction: (i32, i32), heat: u32) -> Self {
        Crucible {
            position,
            direction,
            heat,
        }
    }
}

fn index(direction: &(i32, i32)) -> usize {
    match direction {
        (1, 0) => 0,
        (-1, 0) => 1,
        (0, 1) => 2,
        (0, -1) => 3,
        _ => panic!(),
    }
}

// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Using_a_priority_queue
fn dijkstra(blocks: &[Vec<u32>], least: usize, most: usize) -> u32 {
    let (w, h) = (blocks[0].len(), blocks.len());

    let mut heap = BinaryHeap::new();
    let mut heats = vec![vec![[std::u32::MAX; 4]; w]; h];

    // start
    heats[0][0] = [0, 0, 0, 0];
    heap.push(Crucible::new((0, 0), (0, 0), 0));

    while let Some(crucible) = heap.pop() {
        // end
        if crucible.position == (w as i32 - 1, h as i32 - 1) {
            return crucible.heat;
        }

        // get all next positions
        for d in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if d == crucible.direction || d == (-crucible.direction.0, -crucible.direction.1) {
                continue;
            }

            let mut heat = crucible.heat;
            let (mut x, mut y) = crucible.position;

            for i in 0..most {
                x += d.0;
                y += d.1;

                if x < 0 || y < 0 {
                    break;
                }

                let (x, y) = (x as usize, y as usize);

                if x >= w || y >= h {
                    break;
                }

                heat += blocks[y][x];

                if i + 1 < least {
                    continue;
                }

                if heat < heats[y][x][index(&d)] {
                    heats[y][x][index(&d)] = heat;

                    heap.push(Crucible::new((x as i32, y as i32), d, heat));
                }
            }
        }
    }
    panic!();
}

fn resolve<T>(lines: Lines<T>) -> (u32, u32)
where
    T: BufRead,
{
    let blocks = lines
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| (c as u8 - b'0') as u32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (dijkstra(&blocks, 0, 3), dijkstra(&blocks, 4, 10))
}

#[test]
fn check() {
    const TEST: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (102, 94));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2023::Day::new(file!(), resolve_string) }
