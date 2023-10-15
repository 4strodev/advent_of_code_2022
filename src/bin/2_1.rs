// Link to exercise
// https://adventofcode.com/2022/day/2

#[derive(Debug)]
enum GameOption {
    ROCK(i32),
    PAPER(i32),
    SCISSORS(i32),
}

fn main() {
    // Reading file input
    let input = std::fs::read_to_string("src/assets/2_1_input.txt").unwrap();
    // Getting total games from input file
    let total_games: Vec<_> = input.trim().split("\n").map(|line| {
        let split_line: Vec<_> = line.trim().split(" ").collect();
        return (convert_input(split_line[0]), convert_input(split_line[1]));
    }).collect();

    let total_points = total_games.iter().fold(0, |accum, game| {
        accum + match game {
           (GameOption::ROCK(_), player_option) => {
                match player_option {
                    GameOption::SCISSORS(points) => points + 0,
                    GameOption::ROCK(points) => points + 3,
                    GameOption::PAPER(points) => points + 6
                }
           } 
           (GameOption::PAPER(_), other_option) => {
                match other_option {
                    GameOption::ROCK(points) => points + 0,
                    GameOption::PAPER(points) => points + 3,
                    GameOption::SCISSORS(points) => points + 6,
                }
           } 
           (GameOption::SCISSORS(_), other_option) => {
                match other_option {
                    GameOption::PAPER(points) => points + 0,
                    GameOption::SCISSORS(points) => points + 3,
                    GameOption::ROCK(points) => points + 6,
                }
           } 
        }
    });

    println!("{}", total_points);
}

fn convert_input(input: &str) -> GameOption {
    match input {
        "A" | "X" => GameOption::ROCK(1),
        "B" | "Y" => GameOption::PAPER(2),
        "C" | "Z" => GameOption::SCISSORS(3),
        _ => panic!("Invalid opponent input '{}'", input)
    }
}
