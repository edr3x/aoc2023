mod reader;

fn main() {
    let file = reader::read_file("01.txt");
    println!("Part1 answer: {}", part1(&file));
    println!("Part2 answer: {}", part2(&file));
}

fn part1(file_data: &str) -> i32 {
    let data = file_data
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|&x| {
            x.chars()
                .filter(|&c| c.is_ascii_digit())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let first = data
        .iter()
        .map(|x| x.first().expect(""))
        .collect::<Vec<_>>();
    let last = data.iter().map(|x| x.last().expect("")).collect::<Vec<_>>();

    first
        .iter()
        .zip(last.iter())
        .map(|(&f, &l)| format!("{}{}", f, l).parse().expect("failed to parse"))
        .collect::<Vec<_>>()
        .iter()
        .sum()
}

fn part2(file_data: &str) -> i32 {
    let mut res = 0;
    for line in file_data.lines() {
        let nums = extract_numbers(line);
        res += nums[0] * 10 + nums[1];
    }
    res
}

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn extract_numbers(n: &str) -> [i32; 2] {
    let mut first = None;
    let mut last = 0;

    let mut digit = |c| {
        first = first.or(Some(c));
        last = c;
    };

    let chars = n.as_bytes();
    for (i, c) in chars.iter().enumerate() {
        if c.is_ascii_digit() {
            digit((c - b'0') as i32);
        } else {
            for (j, d) in NUMBERS.iter().enumerate() {
                if chars[i..].starts_with(d.as_bytes()) {
                    digit(j as i32 + 1);
                }
            }
        }
    }

    [first.unwrap(), last]
}

#[cfg(test)]
mod test {
    const EXAMPLE_A: &str = indoc::indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        "};

    const EXAMPLE_B: &str = indoc::indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        "};

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(EXAMPLE_A), 142);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(EXAMPLE_B), 281);
    }
}
