use adventofcode::*;

pub fn main() -> Result<(), anyhow::Error> {
    let passports = read_passport_input_fields("input/day4_1.input");

    let required_fields: Vec<(&str, fn(&str) -> bool)> = vec![
        ("byr", |s| {
            let number = s.parse::<i64>().unwrap();
            number >= 1920 && number <= 2002 && s.len() == 4
        }),
        ("iyr", |s| {
            let number = s.parse::<i64>().unwrap();
            number >= 2010 && number <= 2020 && s.len() == 4
        }),
        ("eyr", |s| {
            let number = s.parse::<i64>().unwrap();
            number >= 2020 && number <= 2030 && s.len() == 4
        }),
        ("hgt", |s| {
            let is_cm = s.ends_with("cm");
            let is_in = s.ends_with("in");

            if !(is_cm || is_in) {
                return false;
            }

            let number_part = &s[0..s.len() - 2];
            let number = number_part.parse::<i64>().unwrap();
            if is_cm {
                number >= 150 && number <= 193
            } else {
                number >= 59 && number <= 76
            }
        }),
        ("hcl", |s| {
            let code_part = &s[1..s.len()];
            s.len() == 7
                && s.starts_with("#")
                && code_part.as_bytes().iter().all(u8::is_ascii_alphanumeric)
        }),
        ("ecl", |s| match s {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        }),
        ("pid", |s| {
            s.len() == 9
                && s.as_bytes()
                    .iter()
                    .map(|byte| *byte as char)
                    .all(char::is_numeric)
        }),
    ];

    let mut valid_passports = 0;

    for passport in &passports {
        let mut valid = 1;
        for (required_field, validation_fn) in &required_fields {
            if let Some(field) = passport.get(*required_field) {
                if !validation_fn(&field) {
                    valid = 0;
                    continue;
                }
            } else {
                valid = 0;
                continue;
            }
        }

        valid_passports += valid;
    }

    println!("Solution: {}", valid_passports);
    Ok(())
}
