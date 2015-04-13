use oauth2::ApplicationSecret;

use std::fs;
use std::env;
use std::io;
use std::path::Path;

#[derive(Debug)]
pub enum ArgumentError {
    ConfigurationDirectoryInaccessible(io::Error),
    ConfigurationDirectoryUnset,
    UsernameExpansionFailed(String),
}

#[derive(Debug)]
pub struct InvalidOptionsError {
    pub issues: Vec<ArgumentError>,
    pub exit_code: i32,
}

impl InvalidOptionsError {
    pub fn single(err: ArgumentError, exit_code: i32) -> InvalidOptionsError {
        InvalidOptionsError {
            issues: vec![err],
            exit_code: exit_code,
        }
    }
}

pub fn assure_config_dir_exists(dir: &str) -> Result<String, ArgumentError> {
    let trdir = dir.trim();
    if trdir.len() == 0 {
        return Err(ArgumentError::ConfigurationDirectoryUnset)
    }

    let expanded_config_dir = 
        if trdir.as_bytes()[0] == b'~' {
            match env::var("HOME").ok().or(env::var("UserProfile").ok()) {
                None => return Err(ArgumentError::UsernameExpansionFailed(trdir.to_string())),
                Some(mut user) => {
                    user.push_str(&trdir[1..]);
                    user
                }
            }
        } else {
            trdir.to_string()
        };

    if let Err(err) = fs::create_dir(&expanded_config_dir) {
        if err.kind() != io::ErrorKind::AlreadyExists {
            return Err(ArgumentError::ConfigurationDirectoryInaccessible(err))
        }
    }

    Ok(expanded_config_dir)
}

// pub fn application_secret_from_directory(dir: String) -> Result<ApplicationSecret, io::Error> {

// }