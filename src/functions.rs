use crate::{
    Options, When, S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR,
};
use chrono::{DateTime, Local};
use std::{
    fs::{self, Metadata},
    os::unix::fs::MetadataExt,
    path::PathBuf,
    time::{Duration, UNIX_EPOCH},
};
use users::{get_group_by_gid, get_user_by_uid};

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
pub fn list_all(directory_path: &PathBuf, options: &Options) -> Result<(), std::io::Error> {
    let entries = fs::read_dir(directory_path)?; // propagate error to the function

    // iterate through all entries
    for entry in entries {
        let entry = entry?;
        let entry_name = entry
            .file_name()
            .to_str()
            .expect("OS String cannot be converted to String")
            .to_owned();
        let metadata = entry.metadata()?; // extract file metadata

        // Skip hidden files if not specified
        if !options.show_all && entry_name.starts_with(".") {
            continue;
        }

        if options.long_format {
            // list file in long format
            let info = list_long_format(&metadata);
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
pub fn list_long_format(metadata: &Metadata) -> String {
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

    // File size
    result.push_str(&format!(" {:5}", metadata.size()));

    // Modification time
    let m_time = UNIX_EPOCH + Duration::from_secs(metadata.mtime() as u64);
    let datetime: DateTime<Local> = DateTime::from(m_time);
    result.push_str(&format!(" {}", datetime.format("%b %d %H:%M")));

    result
}
