use adventofcode::*;

pub fn main() -> Result<(), anyhow::Error> {
    let numbers = read_input_as_list_of_sorted_integers("input/day1_1.input")?;

    let mut right_index = numbers.len() - 1;
    let mut left_index = 0;

    while right_index > left_index {
        let sum = numbers[left_index] + numbers[right_index];

        if sum == 2020 {
            println!("{} * {}", numbers[left_index], numbers[right_index]);
            println!("Result: {}", numbers[left_index] * numbers[right_index]);
            return Ok(());
        }

        if sum > 2020 {
            right_index -= 1;
        } else {
            left_index += 1;
        }
    }

    println!("No solution?");

    Ok(())
}
