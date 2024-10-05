# rls

A Rust implementation of the classic Unix `ls` command.

## Project Overview

`rls` aims to recreate the functionality of the `ls` command using Rust. This project serves as a learning exercise.

> Note: This is a personal project and is not intended to replace the existing `ls` command, I am not that gifted.

## Features

The goal is to implement the following `ls` options:

- [x] `rls`: List all non-hidden files and directories in the current directory
- [x] `rls -a`: List all files and directories, including hidden ones
- [x] `rls -A`: List all files and directories except `.` and `..`
- [x] `rls -l`: Use long listing format
- [x] `rls -h`: Print sizes in human-readable format (e.g., 1K, 234M, 2G)
- [ ] `rls -R`: List subdirectories recursively
- [x] `rls -d`: List directories themselves, not their contents
- [ ] `rls -r`: Reverse order while sorting
- [ ] `rls -S`: Sort by file size, largest first
- [ ] `rls -t`: Sort by modification time, newest first
- [ ] `rls -X`: Sort alphabetically by entry extension
- [ ] `rls -1`: List one file per line
- [ ] `rls --color[=WHEN]`: Colorize the output; WHEN can be 'always' (default if ommited), 'auto', or 'never'
- [ ] `rls -F`: Append indicator (one of */=>@|) to entries
- [ ] `rls -i`: Print the index number of each file
- [ ] `rls -n`: List numeric user and group IDs
- [ ] `rls -p`: Append / indicator to directories

## Roadmap

The following roadmap outlines the planned development stages for the `rls` project. Each step includes a status indicator to show the current progress:

- 🚀 In Progress, ✅ Completed, 🔜 Planned, 🔄 Under Review

1. ✅ Implement basic file listing functionality
2. ✅ Add support for command-line arguments
3. 🚀 Implement sorting options
4. 🔜 Add support for different output formats
5. 🔜 Implement advanced features (recursive listing, colorization, etc.)
6. 🔜 Optimize performance
7. 🔜 Add comprehensive error handling
8. 🔜 Write unit and integration tests

The status of each step will be updated as the project progresses.

## Contributing

While this is primarily a personal project, suggestions and feedback are welcome. Please open an issue to discuss potential changes or improvements.

## License

[MIT License](LICENSE)
