use std::{env, fs, path::PathBuf};

#[derive(Default, Debug)]
struct Options {
    show_all: bool,
    long_format: bool,
    almost_all: bool,
    human_readable: bool,
    recursive: bool,
    directory: bool,
    reverse: bool,
    size_sort: bool, // Sort file by size, largest first
    time_sort: bool, // Sort file by modification time, newest first
    alphabetic_sort: bool,
    one_file_per_line: bool,
    color_when: bool,
    classify: bool,
    inode: bool,
    numeric_uid_gid: bool,
    append_indicator: bool,
}

impl Options {
    fn new() -> Self {
        Options::default()
    }
}

fn main() {
    // Collect arguments
    let args: Vec<String> = env::args().collect();
    let mut options = Options::new();

    //: Get the current working directory
    let directory_path = match env::current_dir() {
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
            "-a" => options.show_all = true,
            "-l" => options.long_format = true,
            "-A" => options.almost_all = true,
            "-h" => options.human_readable = true,
            "-R" => options.recursive = true,
            "-d" => options.directory = true,
            "-r" => options.reverse = true,
            "-S" => options.size_sort = true,
            "-t" => options.time_sort = true,
            "-X" => options.alphabetic_sort = true,
            "-1" => options.one_file_per_line = true,
            "--color[=when]" => options.color_when = true,
            "-F" => options.classify = true,
            "-i" => options.inode = true,
            "-n" => options.numeric_uid_gid = true,
            "-p" => options.append_indicator = true,
            _ => println!(
                "Unknown option: {}, listing all non hidden files and directories",
                arg
            ),
        }
    }

    println!("options: {:#?}", options);

    match list_all(directory_path, options) {
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
fn list_all(directory_path: PathBuf, options: Options) -> Result<(), std::io::Error> {
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
