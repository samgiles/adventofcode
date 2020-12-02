use adventofcode::*;

pub fn main() -> Result<(), anyhow::Error> {
    let password_iter = read_password_input("input/day2_1.input");

    let mut valid_passwords = 0;
    for password in password_iter {
        let i = password.lowest as usize - 1;
        let j = password.highest as usize - 1;
        let chars = password.password.as_bytes();

        let is_valid =
            (chars[i] as char == password.character) ^ (chars[j] as char == password.character);

        if is_valid {
            valid_passwords += 1
        }
    }

    println!("Solution: {}", valid_passwords);

    Ok(())
}
