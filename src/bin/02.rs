use itertools::Itertools;
use std::{collections::HashMap};

#[derive(Eq, PartialEq, Hash, Debug)]
enum Result {
    Rock,
    Paper,
    Scissor,
}

fn get_poin_table() -> HashMap<Result, i32> {
    let mut point_table = HashMap::new();
    point_table.insert(Result::Rock, 1);
    point_table.insert(Result::Paper, 2);
    point_table.insert(Result::Scissor, 3);
    point_table
}

pub fn part_one(input: &str) -> Option<u32> {
    let point_table = get_poin_table();

    let final_point: i32 = input
        .lines()
        .map(|l| {
            let single_res = l
                .split_whitespace()
                .map(|s| match s {
                    "A" | "X" => Result::Rock,
                    "B" | "Y" => Result::Paper,
                    "C" | "Z" => Result::Scissor,
                    _ => panic!("Invalid input"),
                })
                .collect_tuple();

            if let Some((other, me)) = single_res {
                match (other, me) {
                    (x, y) if x == y => *(point_table.get(&x).unwrap()) + 3,
                    (x, y) => {
                        let x_point = point_table.get(&x).unwrap();
                        let y_point = point_table.get(&y).unwrap();
                        if y_point - x_point == 1 || y_point - x_point == -2 {
                            y_point + 6
                        } else {
                            *y_point
                        }
                    }
                }
            } else {
                panic!("Invalid input");
            }
        })
        .sum();

    u32::try_from(final_point).ok()
}

enum ResultNum {
    Move(isize),
}

fn get_strategy_move(opponent_move: ResultNum, step: isize) -> ResultNum {
    let ResultNum::Move(move_value) = opponent_move;
    let mut strategy_move_value = (move_value + step) % 3;
    strategy_move_value = if strategy_move_value == 0 {
        3
    } else {
        strategy_move_value
    };
    ResultNum::Move(strategy_move_value)
}

pub fn part_two(input: &str) -> Option<u32> {
    let final_point: isize = input
        .lines()
        .map(|l| {
            let mut guide_iter = l.split_whitespace();
            let opponent_move = match guide_iter.next().unwrap() {
                "A" => ResultNum::Move(1), // Rock
                "B" => ResultNum::Move(2), // Paper
                "C" => ResultNum::Move(3), // Scissor
                _ => panic!("Invalid input"),
            };
            let my_move = match guide_iter.next().unwrap() {
                "Y" => (opponent_move, 3),
                "X" => (get_strategy_move(opponent_move, -1), 0),
                "Z" => (get_strategy_move(opponent_move, 1), 6),
                _ => panic!("Invalid input"),
            };

            let (ResultNum::Move(move_point), match_point) = my_move;
            move_point + match_point
        })
        .sum();

    u32::try_from(final_point).ok()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
