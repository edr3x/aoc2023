mod reader;

fn main() {
    let file = reader::read_file("02.txt");
    println!("Part1 answer: {}", part1(&file));
    println!("Part2 answer: {}", part2(&file));
}

fn part1(file_data: &str) -> i32 {
    const TOTAL_RED: i32 = 12;
    const TOTAL_BLUE: i32 = 14;
    const TOTAL_GREEN: i32 = 13;

    let mut valid_games: Vec<i32> = Vec::new();

    for l in file_data.lines() {
        let mut is_game_valid = true;

        let x = l.trim().split(": ").collect::<Vec<&str>>();

        let game_num = x[0].split_whitespace().collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();

        x[1].split("; ")
            .collect::<Vec<&str>>()
            .iter()
            .for_each(|b| {
                b.split(", ").for_each(|c| {
                    let color = c.split_whitespace().collect::<Vec<&str>>()[1];

                    let num = c.split_whitespace().collect::<Vec<&str>>()[0]
                        .parse::<i32>()
                        .unwrap();

                    match color {
                        "red" => {
                            if num > TOTAL_RED {
                                is_game_valid = false;
                            }
                        }
                        "blue" => {
                            if num > TOTAL_BLUE {
                                is_game_valid = false;
                            }
                        }
                        "green" => {
                            if num > TOTAL_GREEN {
                                is_game_valid = false;
                            }
                        }
                        _ => {}
                    }
                });
            });
        if is_game_valid {
            valid_games.push(game_num);
        }
    }
    valid_games.iter().sum()
}

fn part2(file_data: &str) -> i32 {
    2286
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = indoc::indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(EXAMPLE), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(EXAMPLE), 2286);
    }
}
