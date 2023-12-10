use std::io::{BufRead, Lines};

fn get_next_direction(direction: (i32, i32), c: char) -> (i32, i32) {
    match (direction, c) {
        // DOWN
        ((0, 1), '|') => (0, 1),
        ((0, 1), 'J') => (-1, 0),
        ((0, 1), 'L') => (1, 0),
        // UP
        ((0, -1), '|') => (0, -1),
        ((0, -1), '7') => (-1, 0),
        ((0, -1), 'F') => (1, 0),
        // RIGHT
        ((1, 0), '-') => (1, 0),
        ((1, 0), 'J') => (0, -1),
        ((1, 0), '7') => (0, 1),
        // LEFT
        ((-1, 0), '-') => (-1, 0),
        ((-1, 0), 'L') => (0, -1),
        ((-1, 0), 'F') => (0, 1),
        _ => panic!(),
    }
}

fn get_start_pipe(start_direction: (i32, i32), end_direction: (i32, i32)) -> char {
    match (start_direction, end_direction) {
        ((0, -1), (-1, 0)) => 'L',
        ((0, -1), (0, -1)) => '|',
        ((0, -1), (1, 0)) => 'J',

        ((1, 0), (0, -1)) => 'F',
        ((1, 0), (1, 0)) => '-',

        ((0, 1), (1, 0)) => '7',
        _ => panic!(),
    }
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut grid = vec![];
    let mut start = (0, 0);

    for line in lines {
        let line = line.unwrap();
        let chars = line.chars().collect::<Vec<_>>();

        let index = chars.iter().position(|&c| c == 'S');
        if let Some(index) = index {
            start = (index, grid.len());
        }

        grid.push(chars);
    }

    let start_direction;

    // find next position to start
    if start.1 > 0 && ['7', '|', 'F'].contains(&grid[start.1 - 1][start.0]) {
        start_direction = (0, -1);
    } else if start.0 < grid[0].len() - 1 && ['7', '-', 'J'].contains(&grid[start.1][start.0 + 1]) {
        start_direction = (1, 0);
    } else if start.1 < grid.len() - 1 && ['J', '|', 'L'].contains(&grid[start.1 + 1][start.0]) {
        start_direction = (0, 1);
    } else if start.0 > 0 && ['F', '-', 'L'].contains(&grid[start.1][start.0 - 1]) {
        start_direction = (-1, 0);
    } else {
        panic!();
    }

    let mut direction = start_direction;
    let mut current = (start.0 as i32 + direction.0, start.1 as i32 + direction.1);
    let mut grid2 = vec![vec!['.'; grid[0].len()]; grid.len()];
    let mut part1 = 1;

    loop {
        let pipe = grid[current.1 as usize][current.0 as usize];

        if pipe == 'S' {
            break;
        }

        grid2[current.1 as usize][current.0 as usize] = pipe;

        direction = get_next_direction(direction, pipe);

        current = (current.0 + direction.0, current.1 + direction.1);

        part1 += 1;
    }

    // replace the S by the real pipe
    grid2[start.1][start.0] = get_start_pipe(start_direction, direction);

    let mut part2 = 0;

    for y in 0..grid2.len() {
        let mut is_inside = false;
        let mut entered_pipe = ' ';

        for x in 0..grid2[0].len() {
            let pipe = grid2[y][x];

            match pipe {
                '.' => {
                    if is_inside {
                        part2 += 1
                    }
                }
                '|' => is_inside = !is_inside,
                'L' | 'F' => entered_pipe = pipe,
                'J' => {
                    if entered_pipe == 'F' {
                        is_inside = !is_inside
                    }
                }
                '7' => {
                    if entered_pipe == 'L' {
                        is_inside = !is_inside
                    }
                }
                '-' => {}
                _ => panic!(),
            }
        }
    }

    (part1 / 2, part2)
}

#[test]
fn check() {
    const TEST1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";
    const TEST2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    const TEST3: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    const TEST4: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
    const TEST5: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST1).lines()), (4, 1));
    assert_eq!(resolve(Cursor::new(TEST2).lines()), (8, 1));
    assert_eq!(resolve(Cursor::new(TEST3).lines()), (23, 4));
    assert_eq!(resolve(Cursor::new(TEST4).lines()), (70, 8));
    assert_eq!(resolve(Cursor::new(TEST5).lines()), (80, 10));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2023::Day::new(file!(), resolve_string) }
