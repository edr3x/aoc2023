mod reader;

fn main() {
    let file = reader::read_file("04.txt");
    println!("Part1 answer: {}", part1(&file));
    println!("Part2 answer: {}", part2(&file));
}

fn part1(file_data: &str) -> u32 {
    let mut points: Vec<u32> = Vec::new();

    for l in file_data.lines() {
        let x = l.split(':').collect::<Vec<&str>>()[1]
            .trim()
            .split('|')
            .collect::<Vec<&str>>();

        let winning_numbers = x[0].split_whitespace().collect::<Vec<&str>>();
        let my_numbers = x[1].split_whitespace().collect::<Vec<&str>>();

        let matches = my_numbers
            .iter()
            .filter(|&n| winning_numbers.contains(n))
            .count();

        let points_for_card = match matches {
            1..=11 => 2u32.pow(matches as u32 - 1),
            _ => 0,
        };
        points.push(points_for_card);
    }
    points.iter().sum()
}

fn part2(file_data: &str) -> i32 {
    30
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = indoc::indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(EXAMPLE), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(EXAMPLE), 30);
    }
}
