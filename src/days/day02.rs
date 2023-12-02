use std::io::{BufRead, Lines};

struct Bag {
    r: u32,
    g: u32,
    b: u32,
}

fn bag_part(line: &str) -> (bool, u32)
{
    const BAG: Bag = Bag { r: 12, g: 13, b: 14 };
    let games = line.split(": ").nth(1).unwrap();
    let mut result = true;
    let mut min_bag = Bag { r: 0, g: 0, b: 0 };

    for game in games.split("; ") {
        for color in game.split(", ") {
            let split = color.split(' ').collect::<Vec<_>>();
            let n = split[0].parse::<u32>().unwrap();
            let (min_bag_n, bag_n) =  match split[1] {
                "red" => { (&mut min_bag.r, BAG.r) }
                "green" => { (&mut min_bag.g, BAG.g) }
                "blue" => { (&mut min_bag.b, BAG.b) }
                _ => panic!(),
            };

            *min_bag_n = (*min_bag_n).max(n);

            if n > bag_n {
                result = false;
            }
        }
    }

    (result, min_bag.r * min_bag.g * min_bag.b)
}

fn resolve<T>(lines: Lines<T>) -> (u32, u32)
where
    T: BufRead,
{
    lines.enumerate().fold((0, 0), |(part1, part2), (i, line)| {
        let line = line.unwrap();
        let (ok, power) = bag_part(&line);
        let index = if ok { i as u32 + 1 } else { 0 };

        (part1 + index, part2 + power)
    })
}

#[test]
fn check() {
    const TEST: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    use std::io::Cursor;

    let (part1, part2) = resolve(Cursor::new(TEST).lines());
    assert_eq!(part1, 8);
    assert_eq!(part2, 2286);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2023::Day::new(file!(), resolve_string) }
