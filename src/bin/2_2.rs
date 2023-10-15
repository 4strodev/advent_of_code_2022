// Link to exercise
// https://adventofcode.com/2022/day/2

type GameOption = i32;
type GameResult = i32;

const ROCK: GameOption = 1;
const PAPER: GameOption = 2;
const SCISSORS: GameOption = 3;

const WIN: GameResult = 6;
const LOSE: GameResult = 0;
const DRAW: GameResult = 3;

fn main() {
    // Reading file input
    let input = std::fs::read_to_string("src/assets/2_input.txt").unwrap();
    // Getting total games from input file
    let total_games: Vec<_> = input
        .trim()
        .split("\n")
        .map(|line| {
            let split_line: Vec<_> = line.trim().split(" ").collect();
            return (
                convert_oponent_input(split_line[0]),
                convert_game_result(split_line[1]),
            );
        })
        .collect();

    let total_points = total_games
        .iter()
        .fold(0, |accum, game| accum + get_points(game));

    println!("{}", total_points);
}

fn convert_oponent_input(input: &str) -> GameOption {
    match input {
        "A" => ROCK,
        "B" => PAPER,
        "C" => SCISSORS,
        _ => panic!("Invalid opponent input '{}'", input),
    }
}

fn convert_game_result(input: &str) -> GameResult {
    match input {
        "X" => LOSE,
        "Y" => DRAW,
        "Z" => WIN,
        _ => panic!("Invalid game input '{}'", input),
    }
}

fn get_points(game: &(GameOption, GameResult)) -> i32 {
    let (opponent_play, game_result) = game;

    if game_result == &WIN {
        winner(opponent_play) + &WIN
    } else if game_result == &DRAW {
        opponent_play + &DRAW
    } else {
        looser(opponent_play) +&LOSE
    }
}

fn winner(option: &GameOption) -> GameOption {
    if option == &ROCK {
        PAPER
    } else if option == &PAPER {
        SCISSORS
    } else {
        ROCK
    }
}

fn looser(option: &GameOption) -> GameOption {
    if option == &ROCK {
        SCISSORS
    } else if option == &PAPER {
        ROCK
    } else {
        PAPER
    }
}
