advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let vec = parse(input);
    Some(vec.windows(2).filter(|pair| pair[0] < pair[1]).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let vec = parse(input);
    let sums: Vec<u64> = vec.windows(3).map(|triple| triple.iter().sum()).collect();
    Some(sums.windows(2).filter(|pair| pair[0] < pair[1]).count() as u32)
}
fn parse(input: &str) -> Vec<u64> {
    input
        .trim_end()
        .lines()
        .map(|l| l.parse().expect("expected u64"))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1832));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1858));
    }
}
