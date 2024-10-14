# rls

A Rust implementation of the classic Unix `ls` command.

## Project Overview

`rls` aims to recreate the functionality of the `ls` command using Rust. This project serves as a learning exercise.

> Note: This is a project for personal studies on how the file system API in rust functions.

## Features

The goal is to implement the following `ls` options:

- [x] `rls`: List all non-hidden files and directories in the current directory
- [x] `rls -a`: List all files and directories, including hidden ones
- [x] `rls -A`: List all files and directories except `.` and `..`
- [x] `rls -l`: Use long listing format
- [x] `rls -h`: Print sizes in human-readable format (e.g., 1K, 234M, 2G)
- [x] `rls -R`: List subdirectories recursively
- [x] `rls -d`: List directories themselves, not their contents
- [x] `rls -r`: Reverse order while sorting
- [x] `rls -S`: Sort by file size, largest first
- [x] `rls -t`: Sort by modification time, newest first
- [x] `rls -X`: Sort alphabetically by entry extension
