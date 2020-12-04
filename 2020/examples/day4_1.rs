use adventofcode::*;

pub fn main() -> Result<(), anyhow::Error> {
    let passports = read_passport_input_fields("input/day4_1.input");

    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut valid_passports = 0;

    for passport in &passports {
        let mut valid = 1;
        for required_field in &required_fields {
            if !passport.contains_key(*required_field) {
                valid = 0;
                continue;
            }
        }

        valid_passports += valid;
    }

    println!("Solution: {}", valid_passports);
    Ok(())
}
