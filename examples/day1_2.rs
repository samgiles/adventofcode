use adventofcode::*;
use std::io;

pub fn main() -> Result<(), anyhow::Error> {
    let mut numbers = read_input_as_list_of_sorted_integers("input/day1_1.input")?;

    println!("{:?}", numbers);

    let last_index = numbers.len() - 1;
    let mut left_index = 0;

    for i in 0..numbers.len() - 2 {
        let mut right_index = last_index;
        let mut left_index = i + 1;

        while left_index < right_index {
            let sum = numbers[i] + numbers[left_index] + numbers[right_index];

            if sum == 2020 {
                println!("{}, {}, {}", numbers[i], numbers[left_index], numbers[right_index]);
                println!("Solution: {}", numbers[i] * numbers[left_index] * numbers[right_index]);
                return Ok(());
            }

            if sum > 2020 {
                right_index -= 1;
            } else {
                left_index += 1;
            }
        }
    }

    println!("No solution?");

    Ok(())
}

