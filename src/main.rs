use std::{env, fs, path::PathBuf};

fn main() {
    // Collect arguments
    let args: Vec<String> = env::args().collect();
    let mut show_all = false;
    let mut long_format = false;

    //: Get the current working directory
    let dir = match env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to get current directory: {}", e);
            std::process::exit(1);
        }
    };

    // load the options from the argument passed in
    // skipping the first argument because it is the program name
    for arg in &args[1..] {
        match arg.as_str() {
            "-a" => show_all = true,
            "-l" => long_format = true,
            _ => println!("Unknown option: {}", arg),
        }
    }

    match list_all(dir, show_all) {
        Ok(()) => {}
        Err(e) => eprintln!("Error listing directory: {}", e),
    };
}

/// A function to list all the entires in a given directory.
///
/// # Arguments
///
/// * `poth` - A `PathBuf`` of the directory file path.
/// * `show_all` - A `bool` to show hidden file or not.
///
/// # Output
///
/// Result of unit type `()` is successful or `std::io::Error` if operation fails.
fn list_all(path: PathBuf, show_all: bool) -> Result<(), std::io::Error> {
    let entries = fs::read_dir(path)?; // propagate error to the function

    // iterate through all entries
    for entry in entries {
        let entry = entry?;
        let entry_name = entry
            .file_name()
            .to_str()
            .expect("OS String cannot be converted to String")
            .to_owned();

        // Skip hidden files if not specified
        if !show_all && entry_name.starts_with(".") {
            continue;
        }

        println!("{}", entry_name);
    }

    Ok(())
}
