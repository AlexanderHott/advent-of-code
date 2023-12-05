fn main() {
    let input = include_str!("../d2p1.txt");
    println!("{}", d2p1(input));
}

const RED: usize = 12;
const GREEN: usize = 13;
const BLUE: usize = 14;

fn d2p1(input: &str) -> usize {
    let mut total = 0;

    for line in input.lines() {
        let (game, cubes) = line.split_once(": ").unwrap();
        let (_, game_id) = game.split_once(" ").unwrap();
        let game_id = game_id.parse::<usize>().unwrap();

        let mut possible = true;
        'game: for round in cubes.split("; ") {
            for color in round.split(", ") {
                possible = match color.split_once(" ").unwrap() {
                    (n, "red") => n.parse::<usize>().unwrap() <= RED,
                    (n, "green") => n.parse::<usize>().unwrap() <= GREEN,
                    (n, "blue") => n.parse::<usize>().unwrap() <= BLUE,
                    _ => unreachable!(),
                };
                if !possible {
                    break 'game;
                }
            }
        }
        if possible {
            total += game_id;
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_pt1() {
        let input = include_str!("../test_input1.txt");
        assert_eq!(8, d2p1(input));
    }

    #[test]
    fn test_pt2() {}
}
