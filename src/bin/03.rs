mod reader;

fn main() {
    let file = reader::read_file("02.txt");
    println!("Part1 answer: {}", part1(&file));
    // println!("Part2 answer: {}", part2(&file));
}

fn part1(file_data: &str) -> i32 {
    8
}

// fn part2(file_data: &str) -> i32 {
//     8
// }

#[cfg(test)]
mod test {
    const EXAMPLE: &str = indoc::indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        "};

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(EXAMPLE), 4361);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(super::part2(EXAMPLE), 2286);
    // }
}
