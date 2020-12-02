use adventofcode::*;

pub fn main() -> Result<(), anyhow::Error> {
    let password_iter = read_password_input("input/day2_1.input");

    let mut valid_passwords = 0;
    for password in password_iter {
        let mut count = 0;
        // Assume ASCII chars.. there must be a better way of doing this..
        for character in password.password.as_bytes() {
            let character = *character as char;
            if character == password.character {
                count += 1;
            }
        }

        if count >= password.lowest && count <= password.highest {
            valid_passwords += 1
        }
    }

    println!("Solution: {}", valid_passwords);

    Ok(())
}
