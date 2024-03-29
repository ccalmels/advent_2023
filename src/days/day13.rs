use advent_2023::Paragrapher;
use std::io::{BufRead, Lines};

#[derive(Debug)]
struct Pattern {
    grid: Vec<Vec<u8>>,
    w: usize,
    h: usize,
}

impl Pattern {
    fn new(grid: Vec<Vec<u8>>) -> Self {
        let w = grid[0].len();
        let h = grid.len();

        Pattern { grid, w, h }
    }

    fn find_vertical(&self) -> (usize, usize) {
        let mut verticals = vec![0; self.w];

        verticals[0] = 2;

        for y in 0..self.h {
            for (index, v) in verticals.iter_mut().enumerate() {
                if *v > 1 {
                    continue;
                }

                *v += (0..index)
                    .rev()
                    .zip(index..self.w)
                    .filter(|&(x1, x2)| self.grid[y][x1] != self.grid[y][x2])
                    .count();
            }
        }

        (
            verticals.iter().position(|&x| x == 0).unwrap_or(0),
            verticals.iter().position(|&x| x == 1).unwrap_or(0),
        )
    }

    fn find_horyzontal(&self) -> (usize, usize) {
        let mut horyzontals = vec![0; self.h];

        horyzontals[0] = 2;

        for x in 0..self.w {
            for (index, v) in horyzontals.iter_mut().enumerate() {
                if *v > 1 {
                    continue;
                }

                *v += (0..index)
                    .rev()
                    .zip(index..self.h)
                    .filter(|&(y1, y2)| self.grid[y1][x] != self.grid[y2][x])
                    .count();
            }
        }

        (
            horyzontals.iter().position(|&y| y == 0).unwrap_or(0),
            horyzontals.iter().position(|&y| y == 1).unwrap_or(0),
        )
    }
}

fn resolve<T>(mut lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    lines
        .split_paragraph(|s| s.as_bytes().to_owned())
        .map(Pattern::new)
        .fold((0, 0), |(p1, p2), pattern| {
            let (v1, v2) = pattern.find_vertical();
            let (h1, h2) = pattern.find_horyzontal();

            (p1 + 100 * h1 + v1, p2 + 100 * h2 + v2)
        })
}

#[test]
fn check() {
    const TEST: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (405, 400));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2023::Day::new(file!(), resolve_string) }
