advent_of_code::solution!(4);
#[derive(Clone)]
enum Field {
    Num(u32),
    Marked,
}
impl Field {
    fn is_marked(&self) -> bool {
        match self {
            Field::Num(_) => false,
            Field::Marked => true,
        }
    }
}
#[derive(Clone)]
struct Board(Vec<Field>);
impl Board {
    fn new(input: &str) -> Board {
        let b = Board(
            input
                .split_ascii_whitespace()
                .map(|s| Field::Num(s.parse().expect("u32")))
                .collect(),
        );
        assert!(b.0.len() == 25);
        b
    }
    fn won(&self) -> bool {
        if self.0.chunks(5).any(|c| c.iter().all(|f| f.is_marked())) {
            // testing horizontal
            return true;
        }
        for i in 0..5 {
            if self
                .0
                .iter()
                .enumerate()
                .filter(|(index, _)| index % 5 == i)
                .all(|(_, f)| f.is_marked())
            {
                return true;
            }
        }

        false
    }
    fn cross(&mut self, x: u32) {
        for f in self.0.iter_mut() {
            match f {
                Field::Num(y) => {
                    if x == *y {
                        *f = Field::Marked;
                    }
                }
                Field::Marked => (),
            }
        }
    }
    fn sum(&self) -> u32 {
        self.0
            .iter()
            .filter_map(|f| match f {
                Field::Num(x) => Some(x),
                Field::Marked => None,
            })
            .sum()
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let (numbers, mut boards) = parse(input);
    for number in numbers {
        for board in &mut boards {
            board.cross(number);
            if board.won() {
                return Some(board.sum() * number);
            }
        }
    }
    unreachable!()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (numbers, mut boards) = parse(input);
    for number in numbers {
        boards.iter_mut().for_each(|b| b.cross(number));
        if boards.len() == 1 && boards.first().unwrap().won() {
            return Some(number * boards.first().unwrap().sum());
        }
        boards.retain(|b| !b.won());
    }
    unreachable!()
}
fn parse(input: &str) -> (Vec<u32>, Vec<Board>) {
    let vec: Vec<_> = input.split("\n\n").collect();
    let numbers = vec.first().expect("non empy vec");
    let numbers = numbers
        .split(',')
        .map(|s| s.parse().expect("u32"))
        .collect();
    let vec = &vec[1..];
    let boards = vec.iter().map(|s| Board::new(s)).collect();
    (numbers, boards)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4_512));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(65_325));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1_924));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(4_624));
    }
}
