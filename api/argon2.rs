#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::nursery, clippy::pedantic)]

use std::error::Error;

use dialoguer::{Password, theme::ColorfulTheme};

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "{}",
        password_auth::generate_hash(
            Password::with_theme(&ColorfulTheme::default())
                .with_prompt("Password")
                .with_confirmation(
                    "Retype password",
                    "Sorry, passwords do not match."
                )
                .interact()?
        )
    );
    Ok(())
}
