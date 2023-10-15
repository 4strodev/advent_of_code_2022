fn main() {
    let input = std::fs::read_to_string("src/assets/1_1_input.txt").unwrap();

    let result = input
        // Removing empty lines at the beggining and the end
        .trim()
        // Getting input lines
        .split("\n")
        // Spaces and newlines for each line
        .map(|line| line.trim())
        // Grouping lines and summing calories
        .fold(
            (Vec::<i32>::new(), 0),
            |(mut result, mut total_elf_calories), line| match line {
                // If we found an empty line that means that a the current counting should be saved
                "" => {
                    // If we count some calories then sum and save
                    if total_elf_calories > 0 {
                        result.push(total_elf_calories);
                    }
                    // Then the following iteration will have a new accumulator
                    (result, 0)
                }
                _ => {
                    let calories = line.parse::<i32>().unwrap();
                    total_elf_calories += calories;
                    (result, total_elf_calories)
                }
            },
        );

    let max_calories = result.0.iter().max().unwrap();

    println!("{:?}", max_calories);
}
