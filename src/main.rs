use std::io::{Write, stdin, stdout};
use std::str::FromStr;

/// Returns the user input parsed to the type T
fn get_user_input<T>() -> Option<T>
where
    T: FromStr,
{
    stdout().flush().ok()?;

    let mut buf = String::new();
    stdin().read_line(&mut buf).ok()?;

    buf.trim().parse::<T>().ok()
}

fn main() {
    println!(
        "Welcome to Zealandia Tracker.\n
        Please choose a task:
        1. Search for bird by common name
        2. Search for bird by scientific name
        3. See all birds in a specific group
        4. Add new classification
        5. Add new species
        6. Exit\n
        Enter a choice (1-6):"
    );

    // Program loop
    loop {
        // Get choice from user until a valid integer is entered
        let choice = loop {
            if let Some(choice) = get_user_input::<u32>() {
                // DEBUG
                println!("{:?}", choice);
                break choice;
            }
            println!("Please enter a number");
        };

        match choice {
            1 => todo!(),
            2 => todo!(),
            3 => todo!(),
            4 => todo!(),
            5 => todo!(),
            6 => break,
            _ => println!("Please enter a number in range (1-6)"),
        }
    }
}
