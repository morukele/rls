/// An Enum to store the possible parameters that can be passed with the --color command
#[derive(Debug)]
pub enum When {
    Always,
    Auto,
    Never,
}

/// A struct to hold all the instructions of the program.
#[derive(Default, Debug)]
pub struct Options {
    pub show_all: bool,
    pub long_format: bool,
    pub almost_all: bool,
    pub human_readable: bool,
    pub recursive: bool,
    pub directory: bool,
    pub reverse: bool,
    pub size_sort: bool, // Sort file by size, largest first
    pub time_sort: bool, // Sort file by modification time, newest first
    pub alphabetic_sort: bool,
    pub one_file_per_line: bool,
    pub color_when: Option<When>, // olorize the output; When can be 'always', 'auto', or 'never'
    pub classify: bool,
    pub inode: bool,
    pub numeric_uid_gid: bool,
    pub append_indicator: bool,
}

impl Options {
    pub fn new() -> Self {
        Options::default()
    }
}
