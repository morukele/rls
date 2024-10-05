/// A file to hold all the permissions constants of the files
// Read, write, and execute by owner.
pub const S_IRUSR: u32 = 0o400; // Read for owner (R)
pub const S_IWUSR: u32 = 0o200; // Write for owner (W)
pub const S_IXUSR: u32 = 0o100; // Execute for owner (X)

// Read, write, and execute by group.
pub const S_IRGRP: u32 = 0o040;
pub const S_IWGRP: u32 = 0o020;
pub const S_IXGRP: u32 = 0o010;

// Read, write, and execute by group.
pub const S_IROTH: u32 = 0o004;
pub const S_IWOTH: u32 = 0o002;
pub const S_IXOTH: u32 = 0o001;
