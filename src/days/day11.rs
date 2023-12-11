use std::io::{BufRead, Lines};

fn compute_free(v: &[bool]) -> Vec<usize> {
    let mut ret = vec![];
    let mut count = 0;

    for i in v.iter() {
        if !*i {
            count += 1;
        }
        ret.push(count);
    }

    ret
}

fn distance(a: usize, b: usize, spaces: &[usize], factor: usize) -> usize {
    let (min, max) = if a < b { (a, b) } else { (b, a) };

    max - min + factor * (spaces[max] - spaces[min])
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut points = vec![];
    let mut rows = vec![];
    let mut columns = vec![];
    let factor = if cfg!(test) { 10 - 1 } else { 1_000_000 - 1 };

    for line in lines {
        let line = line.unwrap();
        let mut row_not_empty = false;

        for (index, c) in line.chars().enumerate() {
            if index >= columns.len() {
                columns.resize(index + 1, false);
            }
            if c == '#' {
                points.push((index, rows.len()));
                columns[index] = true;
                row_not_empty = true;
            }
        }

        rows.push(row_not_empty);
    }

    let rows = compute_free(&rows);
    let columns = compute_free(&columns);
    let (mut part1, mut part2) = (0, 0);

    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            let a = points[i];
            let b = points[j];

            part1 += distance(a.0, b.0, &columns, 1) + distance(a.1, b.1, &rows, 1);
            part2 += distance(a.0, b.0, &columns, factor) + distance(a.1, b.1, &rows, factor);
        }
    }

    (part1, part2)
}

#[test]
fn check() {
    const TEST: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (374, 1030));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2023::Day::new(file!(), resolve_string) }
