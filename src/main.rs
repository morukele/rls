use rls::functions::list_all;
use rls::models::Options;
use std::env;

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
            _ => println!(
                "Unknown option: {}, listing all non hidden files and directories",
                arg
            ),
        }
    }

    // returns 0 for successful run, returns non-zero when an error is encountered
    match list_all(&directory_path, &options) {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            eprintln!("Error listing directory: {}", e);
            std::process::exit(1)
        }
    };
}
