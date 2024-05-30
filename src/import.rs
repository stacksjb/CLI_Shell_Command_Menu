use std::{env, fs, path::Path};

use inquire::Select;

use crate::config::{print_commands, CommandOption, Commands};

pub fn import_commands(config: &mut Commands, changes_made: &mut bool) {
    let dir = env::current_dir().unwrap_or_else(|_| ".".into());
    let files = match list_csv_files(&dir) {
        Ok(files) => files,
        Err(e) => {
            println!("Could not list files: {e}");
            return;
        }
    };
    if files.is_empty() {
        println!("No files found in {}", dir.display());
        return;
    }
    let path = Select::new("Select a file to import: ", files)
        .prompt()
        .expect("Failed to display menu");
    let mut commands = match read_commands(&path) {
        Ok(commands) => commands,
        Err(e) => {
            println!("Could not import file: {e}");
            return;
        }
    };
    print_commands(&commands);

    let menu_options = vec![
        "a. APPEND to current commands",
        "o. OVERWRITE current commands",
        "c. CANCEL import",
    ];
    let menu_prompt = Select::new("Select an option: ", menu_options)
        .prompt()
        .expect("Failed to display menu");
    match menu_prompt {
        "a. APPEND to current commands" => {
            config.commands.append(&mut commands);
            *changes_made = true;
        }
        "o. OVERWRITE current commands" => {
            config.commands = commands;
            *changes_made = true;
        }
        _ => (),
    }
}

fn read_commands(path: &str) -> anyhow::Result<Vec<CommandOption>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut commands = Vec::new();
    for result in reader.deserialize() {
        commands.push(result?);
    }
    Ok(commands)
}

fn list_csv_files(dir_path: &Path) -> anyhow::Result<Vec<String>> {
    let dir_reader = fs::read_dir(dir_path)?;
    let mut files = Vec::new();
    for entry in dir_reader {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            continue;
        }
        let Ok(name) = entry.file_name().into_string() else {
            continue;
        };
        if name.ends_with(".csv") {
            files.push(name);
        }
    }
    Ok(files)
}
