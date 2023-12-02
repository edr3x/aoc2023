mod reader;

fn main() {
    let file = reader::read_file("day01/input.txt");
    part1(&file);
    part2(&file);
}

fn part1(file_data: &str) {
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

    let res: i32 = first
        .iter()
        .zip(last.iter())
        .map(|(&f, &l)| format!("{}{}", f, l).parse().expect("failed to parse"))
        .collect::<Vec<_>>()
        .iter()
        .sum();

    println!("part1 result: {}", res);
}

fn part2(file_data: &str) {
    let mut res = 0;
    for line in file_data.lines() {
        let nums = extract_numbers(line);
        res += nums[0] * 10 + nums[1];
    }

    println!("part2 result: {}", res);
}

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn extract_numbers(n: &str) -> [u32; 2] {
    let mut first = None;
    let mut last = 0;

    let mut digit = |c| {
        first = first.or(Some(c));
        last = c;
    };

    let chars = n.as_bytes();
    for (i, c) in chars.iter().enumerate() {
        if c.is_ascii_digit() {
            digit((c - b'0') as u32);
        } else {
            for (j, d) in NUMBERS.iter().enumerate() {
                if chars[i..].starts_with(d.as_bytes()) {
                    digit(j as u32 + 1);
                }
            }
        }
    }

    [first.unwrap(), last]
}
