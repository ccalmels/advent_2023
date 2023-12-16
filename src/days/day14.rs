use std::io::{BufRead, Lines};

fn go_north(grid: &mut Vec<Vec<char>>) {
    let w = grid[0].len();
    let h = grid.len();

    for x in 0..w {
        let mut count = 0;

        for y in (0..h).rev() {
            let c = &mut grid[y][x];

            if *c == '#' {
                for j in 0..count {
                    grid[y + j + 1][x] = 'O';
                }
                count = 0;
            } else if *c == 'O' {
                *c = '.';
                count += 1;
            }
        }

        for row in grid.iter_mut().take(count) {
            row[x] = 'O';
        }
    }
}

fn go_south(grid: &mut Vec<Vec<char>>) {
    let w = grid[0].len();
    let h = grid.len();

    for x in 0..w {
        let mut count = 0;

        for y in 0..h {
            let c = &mut grid[y][x];

            if *c == '#' {
                for j in 0..count {
                    grid[y - j - 1][x] = 'O';
                }
                count = 0;
            } else if *c == 'O' {
                *c = '.';
                count += 1;
            }
        }

        for j in 0..count {
            grid[h - j - 1][x] = 'O';
        }
    }
}

fn go_west(grid: &mut [Vec<char>]) {
    let w = grid[0].len();

    for row in grid.iter_mut() {
        let mut count = 0;

        for x in (0..w).rev() {
            let c = &mut row[x];

            if *c == '#' {
                for j in 0..count {
                    row[x + j + 1] = 'O';
                }
                count = 0;
            } else if *c == 'O' {
                *c = '.';
                count += 1;
            }
        }

        for c in row.iter_mut().take(count) {
            *c = 'O';
        }
    }
}

fn go_east(grid: &mut [Vec<char>]) {
    let w = grid[0].len();

    for row in grid.iter_mut() {
        let mut count = 0;

        for x in 0..w {
            let c = &mut row[x];

            if *c == '#' {
                for j in 0..count {
                    row[x - j - 1] = 'O';
                }
                count = 0;
            } else if *c == 'O' {
                *c = '.';
                count += 1;
            }
        }

        for j in 0..count {
            row[w - j - 1] = 'O';
        }
    }
}

fn cycle(grid: &mut Vec<Vec<char>>) {
    go_north(grid);
    go_west(grid);
    go_south(grid);
    go_east(grid);
}

fn equal(grid1: &[Vec<char>], grid2: &[Vec<char>]) -> bool {
    let w = grid1[0].len();
    let h = grid1.len();

    for x in 0..w {
        for y in 0..h {
            if grid1[y][x] != grid2[y][x] {
                return false;
            }
        }
    }
    true
}

fn load(grid: &[Vec<char>]) -> usize {
    grid.iter().rev().enumerate().fold(0, |load, (index, row)| {
        load + (index + 1) * row.iter().filter(|&c| *c == 'O').count()
    })
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let initial = lines
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut tortoise = initial.clone();

    go_north(&mut tortoise);

    let part1 = load(&tortoise);

    go_west(&mut tortoise);
    go_south(&mut tortoise);
    go_east(&mut tortoise);

    // https://en.wikipedia.org/wiki/Cycle_detection

    let mut hare = tortoise.clone();
    cycle(&mut hare);

    while !equal(&tortoise, &hare) {
        cycle(&mut tortoise);
        cycle(&mut hare);
        cycle(&mut hare);
    }

    // find start of the loop
    tortoise = initial.clone();
    let mut mu = 0;

    while !equal(&tortoise, &hare) {
        cycle(&mut tortoise);
        cycle(&mut hare);
        mu += 1;
    }

    // find the size of the loop
    let mut lam = 1;

    hare = tortoise.clone();
    cycle(&mut hare);

    while !equal(&tortoise, &hare) {
        cycle(&mut hare);
        lam += 1;
    }

    let rest = (1_000_000_000 - mu) % lam;

    // tortoise = initial.clone();

    // for _ in 0..(mu + rest) {
    //     cycle(&mut tortoise);
    // }

    // tortoise is already at mu cycles
    for _ in 0..rest {
        cycle(&mut tortoise);
    }

    (part1, load(&tortoise))
}

#[test]
fn check() {
    const TEST: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (136, 64));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2023::Day::new(file!(), resolve_string) }
