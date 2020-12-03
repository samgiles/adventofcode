use adventofcode::*;

pub fn main() -> Result<(), anyhow::Error> {
    let ski_slope_map = read_ski_slope_map("input/day3_1.input");

    let rules = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut product = 1;
    for (right, down) in &rules {
        product *= check_tree_count(&ski_slope_map, *right as usize, *down as usize);
    }

    println!("Solution: {}", product);
    Ok(())
}

fn check_tree_count(ski_slope: &SkiSlope, right: usize, down: usize) -> usize {
    let mut position_x = 0;
    let mut position_y = 0;
    let mut trees_encountered = 0;

    while position_y < ski_slope.height {
        if let SkiSlopeSpace::Tree = ski_slope.get_position_wrapping(position_x, position_y) {
            trees_encountered += 1;
        }
        position_x += right;
        position_y += down;
    }

    trees_encountered
}
