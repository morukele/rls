#![allow(non_camel_case_types)]
use std::fmt;

/// A file to hold all the permissions constants of the files
#[derive(Debug, Clone, Copy)]
pub enum Permissions {
    // Read, write, and execute by owner.
    S_IRUSR = 0o400, // Read for owner (R)
    S_IWUSR = 0o200, // Write for owner (W)
    S_IXUSR = 0o100, // Execute for owner (X),

    // Read, write, and execute by group.
    S_IRGRP = 0o040,
    S_IWGRP = 0o020,
    S_IXGRP = 0o010,

    // Read, write, and execute by group.
    S_IROTH = 0o004,
    S_IWOTH = 0o002,
    S_IXOTH = 0o001,
}

impl fmt::Display for Permissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let permission = match self {
            Permissions::S_IRUSR => 0o400,
            Permissions::S_IWUSR => 0o200,
            Permissions::S_IXUSR => 0o100,
            Permissions::S_IRGRP => 0o040,
            Permissions::S_IWGRP => 0o020,
            Permissions::S_IXGRP => 0o010,
            Permissions::S_IROTH => 0o004,
            Permissions::S_IWOTH => 0o002,
            Permissions::S_IXOTH => 0o001,
        };
        write!(f, "{}", permission)
    }
}
