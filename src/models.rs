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
}

impl Options {
    pub fn new() -> Self {
        Options::default()
    }
}
