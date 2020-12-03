use adventofcode::*;

pub fn main() -> Result<(), anyhow::Error> {
    let ski_slope = read_ski_slope_map("input/day3_1.input");

    let mut position_x = 0;
    let mut position_y = 0;
    let mut trees_encountered = 0;

    while position_y < ski_slope.height {
        match ski_slope.get_position_wrapping(position_x, position_y) {
            SkiSlopeSpace::Tree => trees_encountered += 1,
            _ => {}
        };
        position_x += 3;
        position_y += 1;
    }

    println!("Solution: {}", trees_encountered);
    Ok(())
}
