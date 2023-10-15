fn main() {
    let input = std::fs::read_to_string("src/assets/1_1_input.txt").unwrap();

    let result = input
        .trim()
        .split("\n")
        .into_iter()
        .map(|line| line.trim())
        .fold(
            (Vec::<i32>::new(), Vec::<i32>::new()),
            |(mut result, mut current), line| match line {
                "" => {
                    if current.len() > 0 {
                        result.push(current.iter().sum());
                        return (result, Vec::new());
                    }
                    (result, Vec::new())
                }
                _ => {
                    let calories = line.parse::<i32>().unwrap();
                    current.push(calories);
                    (result, current)
                }
            },
        );

    let mut calories_sum = result.0;
    calories_sum.sort_by(|a, b| b.cmp(a));

    println!("{:?}", &calories_sum[0..3].iter().sum::<i32>());
}
