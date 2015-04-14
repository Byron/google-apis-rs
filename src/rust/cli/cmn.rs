use oauth2::{ApplicationSecret, ConsoleApplicationSecret};
use rustc_serialize::json;

use std::fs;
use std::env;
use std::io;
use std::fmt;
use std::path::Path;
use std::cell::RefCell;

use std::io::{Write, Read};

use std::default::Default;

#[derive(Debug)]
pub enum ApplicationSecretError {
    DecoderError((String, json::DecoderError)),
    FormatError(String),    
}

impl fmt::Display for ApplicationSecretError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ApplicationSecretError::DecoderError((ref path, ref err))
                => writeln!(f, "Could not decode file at '{}' with error: {}"
                          , path, err),
            ApplicationSecretError::FormatError(ref path)
                => writeln!(f, "'installed' field is unset in secret file at '{}'"
                            , path),
        }
    }
}

#[derive(Debug)]
pub enum ConfigurationError {
    DirectoryCreationFailed((String, io::Error)),
    DirectoryUnset,
    HomeExpansionFailed(String),
    Secret(ApplicationSecretError),
    IOError((String, io::Error)),
}

impl fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ConfigurationError::DirectoryCreationFailed((ref dir, ref err))
                => writeln!(f, "Directory '{}' could not be created with error: {}", dir, err),
            ConfigurationError::DirectoryUnset 
                => writeln!(f, "--config-dir was unset or empty"),
            ConfigurationError::HomeExpansionFailed(ref dir)
                => writeln!(f, "Couldn't find HOME directory of current user, failed to expand '{}'", dir),
            ConfigurationError::Secret(ref err)
                => writeln!(f, "Secret -> {}", err),
            ConfigurationError::IOError((ref path, ref err))
                => writeln!(f, "IO operation failed on path '{}' with error: {}", path, err),
        }
    }
}

#[derive(Debug)]
pub enum CLIError {
    Configuration(ConfigurationError),
}

impl fmt::Display for CLIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            CLIError::Configuration(ref err) => writeln!(f, "Configuration -> {}", err)
        }
    }
}

#[derive(Debug)]
pub struct InvalidOptionsError {
    pub issues: Vec<CLIError>,
    pub exit_code: i32,
}

impl fmt::Display for InvalidOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for issue in &self.issues {
            try!(issue.fmt(f));
        }
        Ok(())
    }
}

impl InvalidOptionsError {
    pub fn single(err: CLIError, exit_code: i32) -> InvalidOptionsError {
        InvalidOptionsError {
            issues: vec![err],
            exit_code: exit_code,
        }
    }
}

pub fn assure_config_dir_exists(dir: &str) -> Result<String, CLIError> {
    let trdir = dir.trim();
    if trdir.len() == 0 {
        return Err(CLIError::Configuration(ConfigurationError::DirectoryUnset))
    }

    let expanded_config_dir = 
        if trdir.as_bytes()[0] == b'~' {
            match env::var("HOME").ok().or(env::var("UserProfile").ok()) {
                None => return Err(CLIError::Configuration(ConfigurationError::HomeExpansionFailed(trdir.to_string()))),
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
            return Err(CLIError::Configuration(
                    ConfigurationError::DirectoryCreationFailed((expanded_config_dir, err))))
        }
    }

    Ok(expanded_config_dir)
}

pub fn application_secret_from_directory(dir: &str, secret_basename: &str) -> Result<ApplicationSecret, CLIError> {
    let secret_path = Path::new(dir).join(secret_basename);
    let secret_str = || secret_path.as_path().to_str().unwrap().to_string();
    let secret_io_error = |io_err: io::Error| {
            Err(CLIError::Configuration(ConfigurationError::IOError(
                (secret_str(), io_err)
            )))
    };

    for _ in 0..2 {
        match fs::File::open(&secret_path) {
            Err(mut err) => {
                if err.kind() == io::ErrorKind::NotFound {
                    // Write our built-in one - user may adjust the written file at will
                    let secret = ApplicationSecret {
                        client_id: "14070749909-vgip2f1okm7bkvajhi9jugan6126io9v.apps.googleusercontent.com".to_string(),
                        client_secret: "UqkDJd5RFwnHoiG5x5Rub8SI".to_string(),
                        token_uri: "https://accounts.google.com/o/oauth2/token".to_string(),
                        auth_uri: Default::default(),
                        redirect_uris: Default::default(),
                        client_email: None,
                        auth_provider_x509_cert_url: None,
                        client_x509_cert_url: Some("https://www.googleapis.com/oauth2/v1/certs".to_string())
                    };

                    let app_secret = ConsoleApplicationSecret {
                        installed: Some(secret),
                        web: None,
                    };

                    let json_enocded_secret = json::encode(&app_secret).unwrap();
                    err = match fs::OpenOptions::new().create(true).write(true).open(&secret_path) {
                        Err(cfe) => cfe,
                        Ok(mut f) => {
                            match f.write(json_enocded_secret.as_bytes()) {
                                Err(io_err) => io_err,
                                Ok(_) => continue,
                            }
                        }
                    };
                    // fall through to IO error handling
                }
                return secret_io_error(err)
            },
            Ok(mut f) => {
                let mut json_encoded_secret = String::new();
                if let Err(io_err) = f.read_to_string(&mut json_encoded_secret) {
                    return secret_io_error(io_err)
                }
                match json::decode::<ConsoleApplicationSecret>(&json_encoded_secret) {
                    Err(json_decode_error) => return Err(CLIError::Configuration(
                        ConfigurationError::Secret(ApplicationSecretError::DecoderError(
                                                (secret_str(), json_decode_error)
                                              )))),
                    Ok(console_secret) => match console_secret.installed {
                        Some(secret) => return Ok(secret),
                        None => return Err(
                                    CLIError::Configuration(
                                    ConfigurationError::Secret(
                                    ApplicationSecretError::FormatError(secret_str())
                                    )))
                    },
                }
            }
        }
    }
    unreachable!();
}