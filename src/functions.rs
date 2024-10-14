use crate::{
    Options, S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR,
};
use chrono::{DateTime, Local};
use std::{
    cmp::Reverse,
    fs::{self, DirEntry, Metadata},
    os::unix::fs::MetadataExt,
    path::PathBuf,
    time::{Duration, UNIX_EPOCH},
};
use users::{get_group_by_gid, get_user_by_uid};

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
pub fn list_all(directory_path: &PathBuf, options: &Options) -> Result<(), std::io::Error> {
    // if -d flag is set, only list the directory itself
    if options.directory {
        let metadata = fs::metadata(directory_path)?; // get the metadata of the directory itself
        let dir_name = directory_path
            .file_name()
            .unwrap_or_else(|| directory_path.as_os_str())
            .to_string_lossy()
            .into_owned(); // convert directory name from OsString to String

        if options.long_format {
            let info = list_long_format(&metadata, &options.human_readable);
            println!("{} {}", info, dir_name);
        } else {
            println!("{}", dir_name);
        }

        // Early return to get only the directory info
        return Ok(());
    }

    // Iterate though the contents if this -d flag is not set
    let entries = fs::read_dir(directory_path)?; // propagate error to the function

    // If recursive "-R" flag is set, and not in the top level, display the directory name
    if options.recursive {
        println!("\n{}:", directory_path.display());
    }

    // collect directory iterators into a vector for sorting
    let mut entries: Vec<DirEntry> = entries.filter_map(Result::ok).collect();

    // if the -r flag is set
    if options.reverse {
        // had to convert to string here because &[u8] does not implement `Sized` Trait
        entries.sort_by_key(|dir| Reverse(dir.path().to_string_lossy().to_string()));
    }

    // if the -S flag is set, sort by file size, largest first
    if options.size_sort {
        entries.sort_by_key(|dir| {
            Reverse(
                dir.metadata()
                    .expect("Unable to get metadata from file")
                    .size(),
            )
        });
    }

    // if the -t flag is set, sort by modification time, newest first
    if options.time_sort {
        entries.sort_by_key(|dir| {
            Reverse(
                dir.metadata()
                    .expect("unable to get metadata from file")
                    .modified()
                    .expect("unable to get modification time from metadata"),
            )
        });
    }

    // if the -X flag is set, sort alphabetically by entry extension
    if options.alphabetic_sort {
        entries.sort_by_key(|dir| {
            dir.path()
                .extension()
                .unwrap_or_default() // default here will be an empty string
                .to_string_lossy()
                .to_string()
        });
    }

    // iterate through all entries
    for entry in entries {
        let entry_name = entry.file_name().to_string_lossy().into_owned();
        let metadata = entry.metadata()?; // extract file metadata

        // Skip hidden files if not specified
        if (!options.show_all || options.almost_all) && entry_name.starts_with(".") {
            continue;
        }

        // if the entry is a directory, and the "-R" flag is set, recursively print the directory
        if options.recursive && metadata.is_dir() {
            let mut sub_dir_path = directory_path.clone(); // Cloning here because I don't want to temper with the value of the directory_path.
            sub_dir_path.push(&entry_name);

            // Recursively call list_all on the subdirectory
            list_all(&sub_dir_path, options)?;
        }

        if options.long_format {
            // list file in long format, also check if the human readable is set to true
            let info = list_long_format(&metadata, &options.human_readable);
            println!("{} {}", info, entry_name);
        } else {
            println!("{}", entry_name);
        }
    }

    Ok(())
}

/// A function to get the necessary information of the long listing format of ls
///
/// # Arguments
///
/// `metadata`: the metadata of the file or directory
///
/// # Output
///
/// A `String` of permissions.
pub fn list_long_format(metadata: &Metadata, human_readable: &bool) -> String {
    // A helper function to get the permision of each group based on the mode of the file
    // This is done to avoid repetition
    fn get_permission_char(
        mode: &u32,
        read: &u32,
        write: &u32,
        execute: &u32,
    ) -> (char, char, char) {
        (
            if mode & read != 0 { 'r' } else { '-' },
            if mode & write != 0 { 'w' } else { '-' },
            if mode & execute != 0 { 'x' } else { '-' },
        )
    }

    let mut result = String::new();

    // Directory or file
    result.push(if metadata.is_dir() { 'd' } else { '-' });

    // Extract permissions from mode
    let mode = metadata.mode();
    let user_perms = get_permission_char(&mode, &S_IRUSR, &S_IWUSR, &S_IXUSR);
    let group_perms = get_permission_char(&mode, &S_IRGRP, &S_IWGRP, &S_IXGRP);
    let other_perms = get_permission_char(&mode, &S_IROTH, &S_IWOTH, &S_IXOTH);

    // Add permission to the result string
    result.push(user_perms.0);
    result.push(user_perms.1);
    result.push(user_perms.2);

    result.push(group_perms.0);
    result.push(group_perms.1);
    result.push(group_perms.2);

    result.push(other_perms.0);
    result.push(other_perms.1);
    result.push(other_perms.2);

    // Number of hard links
    result.push_str(&format!(" {}", metadata.nlink()));

    // Owner and group
    let owner = get_user_by_uid(metadata.uid())
        .map(|user| user.name().to_string_lossy().into_owned())
        .unwrap_or_else(|| "-".to_string());
    let group = get_group_by_gid(metadata.gid())
        .map(|group| group.name().to_string_lossy().into_owned())
        .unwrap_or_else(|| "-".to_string());
    result.push_str(&format!(" {} {}", owner, group));

    // File size, (human reaable or default)
    let size_output = if *human_readable {
        human_readable_size(metadata.size())
    } else {
        metadata.size().to_string()
    };
    result.push_str(&format!(" {}", size_output));

    // Modification time
    let m_time = UNIX_EPOCH + Duration::from_secs(metadata.mtime() as u64);
    let datetime: DateTime<Local> = DateTime::from(m_time);
    result.push_str(&format!(" {}", datetime.format("%b %d %H:%M")));

    result
}

/// A function to convert the file size from bytes to human readable format
fn human_readable_size(size: u64) -> String {
    let units = ["B", "K", "M", "G", "T"];
    let mut size = size as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < units.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    // Strip trailing ".0" if it's an integer
    if size.fract() == 0.0 {
        format!("{:.0}{}", size, units[unit])
    } else {
        format!("{:.1}{}", size, units[unit])
    }
}
