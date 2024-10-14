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
- [x] `rls -R`: List subdirectories recursively
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
