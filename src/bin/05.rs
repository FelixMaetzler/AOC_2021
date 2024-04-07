use std::{collections::HashMap, ops::RangeInclusive};
advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);
    let input: Vec<_> = input
        .iter()
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
        .collect();
    let mut map = HashMap::new();
    for &((x1, y1), (x2, y2)) in input {
        let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        let all = cartesian_product(x1..=x2, y1..=y2);
        for e in all {
            map.entry(e).and_modify(|v| *v += 1).or_insert(1);
        }
    }
    Some(map.iter().filter(|(_, v)| v >= &&2).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    let input: Vec<_> = input
        .iter()
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2 || x1.abs_diff(*x2) == y1.abs_diff(*y2))
        .collect();
    let mut map = HashMap::new();
    for &((x1, y1), (x2, y2)) in input {
        let all = if x1 == x2 {
            let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            cartesian_product(x1..=x2, y1..=y2)
        } else if y1 == y2 {
            let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            cartesian_product(x1..=x2, y1..=y2)
        } else if x1 < x2 {
            let mut v = vec![];
            if y1 < y2 {
                for x in x1..=x2 {
                    v.push((x, y1 + (x - x1)));
                }
            } else {
                for x in x1..=x2 {
                    v.push((x, y1 - (x - x1)));
                }
            };
            v
        } else {
            let (x1, x2) = (x2, x1);
            let (y1, y2) = (y2, y1);
            let mut v = vec![];
            if y1 < y2 {
                for x in x1..=x2 {
                    v.push((x, y1 + (x - x1)));
                }
            } else {
                for x in x1..=x2 {
                    v.push((x, y1 - (x - x1)));
                }
            };
            v
        };
        for e in all {
            map.entry(e).and_modify(|v| *v += 1).or_insert(1);
        }
    }
    Some(map.iter().filter(|(_, v)| v >= &&2).count() as u32)
}
fn parse(input: &str) -> Vec<((u32, u32), (u32, u32))> {
    input
        .lines()
        .map(|l| l.split_once(" -> ").expect("expect -> "))
        .map(|(p1, p2)| {
            let p1 = p1.split_once(',').expect("expect ,");
            let p1 = (p1.0.parse().expect("u32"), p1.1.parse().expect("u32"));
            let p2 = p2.split_once(',').expect("expect ,");
            let p2 = (p2.0.parse().expect("u32"), p2.1.parse().expect("u32"));
            (p1, p2)
        })
        .collect()
}
fn cartesian_product(x: RangeInclusive<u32>, y: RangeInclusive<u32>) -> Vec<(u32, u32)> {
    x.into_iter()
        .flat_map(|xx| y.clone().map(move |yy| (xx, yy)))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(7_269));
    }
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(21_140));
    }
}
