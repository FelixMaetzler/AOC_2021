advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 80)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 256)
}
fn solve(input: &str, days: u64) -> Option<u64> {
    let vec = parse(input);

    let mut d = vec![];
    for i in 0..=8 {
        d.push(vec.iter().filter(|n| **n == i).count() as u64);
    }
    for _ in 0..days {
        let zeros = d[0];
        for i in 0..8 {
            d[i] = d[i + 1]
        }
        d[6] += zeros;
        d[8] = zeros;
    }
    Some(d.iter().sum())
}
fn parse(input: &str) -> Vec<u64> {
    input
        .trim_end()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5_934));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(372_984));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(26_984_457_539));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1_681_503_251_694));
    }
}
