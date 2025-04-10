mod birds;
mod file;

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
    // build the tree
    let mut tree = birds::build_tree();

    // load json contents into the tree
    file::load_to_tree(&mut tree);

    let message = "Welcome to Zealandia Tracker.\n
            Please choose a task:
            1. Search for bird by common name
            2. Search for bird by scientific name
            3. See all birds in a specific group
            4. Add new classification
            5. Add new species
            6. Exit\n
            Enter a choice (1-6):";

    // Program loop
    loop {
        println!("{}", message);
        // get choice from user until a valid integer is entered
        let choice = loop {
            if let Some(choice) = get_user_input::<u32>() {
                break choice;
            }
            println!("Please enter a number");
        };

        // perform actions on the user's choice
        match choice {
            1 => {
                // search for bird details by name
                println!("Enter the name of the bird:");
                if let Some(name) = get_user_input::<String>() {
                    if let Some(bird) = tree.search_by_name(&name) {
                        println!("\n{}\n", bird);
                    } else {
                        println!("Could not find bird: {}", &name);
                    }
                }
            }
            2 => {
                // search for bird details by scientific name
                println!("Enter the scientific name of the bird:");
                if let Some(name) = get_user_input::<String>() {
                    if let Some(bird) = tree.search_by_scientific_name(&name) {
                        println!("\n{}\n", bird);
                    } else {
                        println!("Could not find bird with scientific name: {}", &name);
                    }
                }
            }
            3 => {
                // get all birds in a group
                println!("Enter the bird group:");
                if let Some(group_name) = get_user_input::<String>() {
                    match tree.birds_in_group_from_name(&group_name) {
                        Ok(birds) => {
                            for bird in birds.iter() {
                                println!("{}\n", bird);
                            }
                        }
                        Err(_) => {
                            println!("There is no group with name: {}", &group_name);
                        }
                    }
                }
            }
            4 => {
                // add a group
                println!("Enter the parent group");
                if let Some(parent_group) = get_user_input::<String>() {
                    println!("Enter the new group name");
                    if let Some(new_group) = get_user_input::<String>() {
                        match tree.add_group(&parent_group, &new_group) {
                            Ok(()) => {
                                println!("Added {}, to {}\n", &new_group, &parent_group);
                            }
                            Err(_) => {
                                println!("There is no group with name: {}", &parent_group);
                            }
                        }
                    }
                }
            }
            5 => {
                // add a bird
                println!("Enter the parent group");
                if let Some(parent_group) = get_user_input::<String>() {
                    println!("Enter the new bird name");
                    if let Some(name) = get_user_input::<String>() {
                        println!("Enter the new bird's scientific name");
                        if let Some(scientific_name) = get_user_input::<String>() {
                            match tree.add_bird(&parent_group, &name, &scientific_name) {
                                Ok(()) => {
                                    println!("Added {}, to {}\n", &name, &parent_group);
                                }
                                Err(_) => {
                                    println!("There is no group with name: {}", &parent_group);
                                }
                            }
                        }
                    }
                }
            }
            // exit the program
            6 => {
                file::save_tree(&tree);
                break;
            }
            _ => println!("Please enter a number in range (1-6)"),
        }
    }
}
