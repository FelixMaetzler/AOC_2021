advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, fuel_part1)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, fuel_part2)
}
fn parse(input: &str) -> Vec<u32> {
    input
        .trim_end()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}
fn fuel_part1(vec: &[u32], pos: u32) -> u32 {
    vec.iter().map(|val| val.abs_diff(pos)).sum()
}
fn fuel_part2(vec: &[u32], pos: u32) -> u32 {
    vec.iter()
        .map(|val| val.abs_diff(pos))
        .map(|n| (n * (n + 1)) / 2)
        .sum()
}
fn solve(input: &str, fuel: impl Fn(&[u32], u32) -> u32) -> Option<u32> {
    let vec = parse(input);
    let min = *vec.iter().min().unwrap();
    let max = *vec.iter().max().unwrap();
    (min..=max).map(|pos| fuel(&vec, pos)).min()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(333_755));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(168));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(94_017_638));
    }
}
