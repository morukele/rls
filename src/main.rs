use rls::functions::list_all;
use rls::models::Options;
use std::env;

const SUCCESS: i32 = 0;
const ERROR: i32 = 1;

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
            "--help" => {
                println!("Welcome to r-ls, a Rust implementation of the ls utility in linux.");
                println!("Usage: rls [Option]. (you can chain options)");
                println!("The following options are available: ");
                println!(" -a: List all files and directories, including hidden ones\n -A: List all files and directories except `.` and `..` \n -l: Use long listing format \n -h: Print sizes in human-readable format (e.g., 1K, 234M, 2G) \n -R: List subdirectories recursively \n -d: List directories themselves, not their contents \n -r: Reverse order while sorting \n -S: Sort by file size, largest first \n -t: Sort by modification time, newest first \n -X: Sort alphabetically by entry extension \n"
                );

                // exit early
                std::process::exit(SUCCESS);
            }
            "-v" | "--version" => {
                println!("v0.1.1");
                std::process::exit(SUCCESS);
            }
            _ => {
                eprintln!(
                    "Unknown option: {}, use the --help flag to see available options",
                    arg
                );
                std::process::exit(ERROR)
            }
        }
    }

    // returns 0 for successful run, returns non-zero when an error is encountered
    match list_all(&directory_path, &options) {
        Ok(()) => std::process::exit(ERROR),
        Err(e) => {
            eprintln!("Error listing directory: {}", e);
            std::process::exit(SUCCESS)
        }
    };
}
