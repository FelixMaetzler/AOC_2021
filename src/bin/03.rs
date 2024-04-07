advent_of_code::solution!(3);
#[derive(Debug, Clone, Copy)]
enum Common {
    One,
    Zero,
    Both,
}
pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);
    let length = input.iter().map(|s| s.len()).min().expect("not empty vec");
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..length {
        let shift = length - i - 1;
        let mut val = 0;
        for s in &input {
            match s.chars().nth(i).expect("inbound") {
                '0' => val -= 1,
                '1' => val += 1,
                _ => unreachable!(),
            }
        }

        match val {
            i32::MIN..=-1 => epsilon += 1 << shift,
            0 => unreachable!(),
            1..=i32::MAX => gamma += 1 << shift,
        }
    }
    Some(gamma * epsilon)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    let length = input.iter().map(|s| s.len()).min().expect("not empty vec");
    let mut new_input = input.clone();
    for i in 0..length {
        let c = more_common_at(&new_input, i);
        let t: Vec<_> = match c {
            Common::One => new_input
                .iter()
                .filter(|s| s.chars().nth(i).expect("inbound") == '1')
                .collect(),
            Common::Zero => new_input
                .iter()
                .filter(|s| s.chars().nth(i).expect("inbound") == '0')
                .collect(),
            Common::Both => new_input
                .iter()
                .filter(|s| s.chars().nth(i).expect("inbound") == '1')
                .collect(),
        };
        new_input = t.iter().map(|s| s.to_string()).collect();
    }
    assert_eq!(new_input.len(), 1);
    let oxygen = u32::from_str_radix(new_input.first().unwrap(), 2).expect("bits");

    new_input = input;
    for i in 0..length {
        let c = more_common_at(&new_input, i);

        let t: Vec<_> = match c {
            Common::One => new_input
                .iter()
                .filter(|s| s.chars().nth(i).expect("inbound") == '0')
                .collect(),
            Common::Zero => new_input
                .iter()
                .filter(|s| s.chars().nth(i).expect("inbound") == '1')
                .collect(),
            Common::Both => new_input
                .iter()
                .filter(|s| s.chars().nth(i).expect("inbound") == '0')
                .collect(),
        };
        new_input = t.iter().map(|s| s.to_string()).collect();
        if new_input.len() == 1 {
            break;
        }
    }
    assert_eq!(new_input.len(), 1);
    let co2 = u32::from_str_radix(new_input.first().unwrap(), 2).expect("bits");
    Some(oxygen * co2)
}
fn more_common_at(input: &[String], i: usize) -> Common {
    let mut val = 0;
    for s in input {
        match s.chars().nth(i).expect("inbound") {
            '0' => val -= 1,
            '1' => val += 1,
            _ => unreachable!(),
        }
    }
    match val {
        i32::MIN..=-1 => Common::Zero,
        0 => Common::Both,
        1..=i32::MAX => Common::One,
    }
}
fn parse(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(198));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(4_174_964));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(230));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(4_474_944));
    }
}
