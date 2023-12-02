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

    println!("Total:{}", res);
}

fn part2(file_data: &str) {
    println!("part2: {}", file_data);
}
