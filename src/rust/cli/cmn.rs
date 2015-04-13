

#[derive(Debug)]
pub enum ArgumentError {
    ConfigurationDirectoryInaccessible(String),
}

#[derive(Debug)]
pub struct InvalidOptionsError {
    pub issues: Vec<ArgumentError>,
    pub exit_code: i32,
}