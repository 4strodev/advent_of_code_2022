// Exercise link
// https://adventofcode.com/2022/day/3
fn main() {
    let input = std::fs::read_to_string("src/assets/3_input.txt").unwrap();
    let lines = input.trim().split("\n").map(|line| line.trim());
    let total_priority: i32 = lines
        .map(|line| {
            let split_size = line.len() / 2;
            let first_split = &line[0..split_size];
            let seccond_split = &line[split_size..line.len()];
            let matching_element = first_split
                .chars()
                .find(|element| seccond_split.contains(*element))
                .unwrap();
            convert_priority(matching_element)
        })
        .sum();

    println!("{}", total_priority);
}

fn convert_priority(character: char) -> i32 {
    if is_lowercase(character) {
        ((character as u8) - b'a' + 1).into()
    } else {
        ((character as u8) - b'A' + 27).into()
    }
}

fn is_lowercase(character: char) -> bool {
    let ascii_value = character as u8;
    ascii_value >= b'a' && ascii_value <= b'z'
}
