use crate::{Options, When};
use std::{fs, path::PathBuf};

/// A function to parsr the --color[=WHEN] into the color option
pub fn parse_color_option(arg: &str) -> Option<When> {
    if arg == "--color" {
        Some(When::Always) // Defaults to "always" if no value is provided
    } else if arg.starts_with("--color=") {
        // strip the argument of it "--color=" characters to get the main color
        arg.strip_prefix("--color=").map(|s| match s {
            "always" => When::Always,
            "auto" => When::Auto,
            "never" => When::Never,
            // Default to always if a wrong argument is passed.
            // Consider a better way to handle this, maybe inducate to the user that
            // the option is wrong and default of "always" is used.
            _ => When::Always,
        })
    } else {
        None
    }
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
pub fn list_all(directory_path: PathBuf, options: Options) -> Result<(), std::io::Error> {
    let entries = fs::read_dir(directory_path)?; // propagate error to the function

    // iterate through all entries
    for entry in entries {
        let entry = entry?;
        let entry_name = entry
            .file_name()
            .to_str()
            .expect("OS String cannot be converted to String")
            .to_owned();

        // Skip hidden files if not specified
        if !options.show_all && entry_name.starts_with(".") {
            continue;
        }

        println!("{}", entry_name);
    }

    Ok(())
}
