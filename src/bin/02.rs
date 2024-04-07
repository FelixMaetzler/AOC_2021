advent_of_code::solution!(2);
use std::str::FromStr;

enum Dir {
    Forward(u32),
    Up(u32),
    Down(u32),
}
impl FromStr for Dir {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, int) = s.split_once(' ').expect("missing space");
        let int = int.parse().expect("u64");
        match s {
            "forward" => Ok(Dir::Forward(int)),
            "up" => Ok(Dir::Up(int)),
            "down" => Ok(Dir::Down(int)),
            _ => Err(format!("{s} doesn't match one of the Dirs")),
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);
    let mut horizontal = 0;
    let mut depth = 0;
    for dir in input {
        match dir {
            Dir::Forward(x) => horizontal += x,
            Dir::Up(x) => depth -= x,
            Dir::Down(x) => depth += x,
        }
    }
    Some(horizontal * depth)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for dir in input {
        match dir {
            Dir::Forward(x) => {
                horizontal += x;
                depth += aim * x
            }
            Dir::Up(x) => aim -= x,
            Dir::Down(x) => aim += x,
        }
    }
    Some(horizontal * depth)
}
fn parse(input: &str) -> Vec<Dir> {
    input
        .lines()
        .map(|l| Dir::from_str(l).expect("Dir"))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(150));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1_804_520));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(900));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1_971_095_320));
    }
}
