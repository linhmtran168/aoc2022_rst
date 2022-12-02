pub fn part_one(input: &str) -> Option<u32> {
    let group = input.split("\n\n");
    let max = group.fold(0, |acc, g| {
        let g_calo: u32 = g.lines().map(|l| l.parse::<u32>().unwrap()).sum();
        if g_calo > acc {
            g_calo
        } else {
            acc
        }
    });
    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let group = input.split("\n\n");
    let mut list_total: Vec<u32> = group.map(|g| {
        g.lines().map(|l| l.parse::<u32>().unwrap()).sum::<u32>()
    }).collect();
    list_total.sort_by(|a, b| b.cmp(a));
    Some(list_total[..3].iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
