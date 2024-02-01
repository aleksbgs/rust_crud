use rand_core::{RngCore, OsRng};
use std::collections::HashSet;

pub(crate) struct PasswordGenerator {
    generated_passwords: HashSet<String>,
}

impl PasswordGenerator {
    pub fn new() -> Self {
        PasswordGenerator {
            generated_passwords: HashSet::new(),
        }
    }

    pub fn generate(&mut self) -> String {
        loop {
            let mut rng = OsRng;
            let mut password = String::new();

            // Generate 7 alphanumeric characters
            for _ in 0..7 {
                let mut byte = [0];
                rng.fill_bytes(&mut byte);
                let char = (byte[0] % 26) as char;
                password.push(if rng.next_u32() % 2 == 0 { char } else { char.to_ascii_uppercase() });
            }

            // Generate a special character (not at the end)
            let special_chars = "!@#$%^&*".as_bytes();
            let special_char = special_chars[(rng.next_u32() as usize) % special_chars.len()] as char;
            let position = (rng.next_u32() as usize) % 7;
            password.insert(position, special_char);

            // Generate an uppercase character (not at the start)
            let uppercase_char = ((rng.next_u32() % 26) as u8 + b'A') as char;
            let position = (rng.next_u32() as usize) % 8 + 1;
            password.insert(position, uppercase_char);

            // Check if the password is unique
            if !self.generated_passwords.contains(&password) {
                self.generated_passwords.insert(password.clone());
                return password;
            }
        }
    }
}