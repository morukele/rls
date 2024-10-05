use rls::functions::{list_all, parse_color_option};
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
            "-1" => options.one_file_per_line = true,
            // option for the --color when
            arg if arg.starts_with("--color") => {
                if let Some(when) = parse_color_option(arg) {
                    options.color_when = Some(when)
                }
            }
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
