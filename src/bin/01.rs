mod reader;

fn main() {
    let file = reader::read_file("day01/example.txt");
    part1(&file);
    part2(&file);
}

fn part1(file_data: &str) {
    println!("part1: {}", file_data);
}

fn part2(file_data: &str) {
    println!("part2: {}", file_data);
}
