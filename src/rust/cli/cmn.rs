use oauth2::{ApplicationSecret, ConsoleApplicationSecret};
use rustc_serialize::json;

use std::fs;
use std::env;
use std::io;
use std::path::Path;

use std::io::{Write, Read};

use std::default::Default;

#[derive(Debug)]
pub enum ArgumentError {
    ConfigurationDirectoryInaccessible((String, io::Error)),
    ConfigurationDirectoryUnset,
    UsernameExpansionFailed(String),
    IOError((String, io::Error)),
    SecretDecoderError((String, json::DecoderError)),
    SecretFormatError(String),
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
            return Err(ArgumentError::ConfigurationDirectoryInaccessible((expanded_config_dir, err)))
        }
    }

    Ok(expanded_config_dir)
}

pub fn application_secret_from_directory(dir: &str, secret_basename: &str) -> Result<ApplicationSecret, ArgumentError> {
    let secret_path = Path::new(dir).join(secret_basename);
    let secret_str = || secret_path.as_path().to_str().unwrap().to_string();
    let secret_io_error = |io_err: io::Error| {
            Err(ArgumentError::IOError(
                (secret_str(), io_err)
            ))
    };

    for _ in 0..2 {
        match fs::File::open(&secret_path) {
            Err(mut e) => {
                if e.kind() == io::ErrorKind::NotFound {
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
                    e = match fs::OpenOptions::new().create(true).write(true).open(&secret_path) {
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
                return secret_io_error(e)
            },
            Ok(mut f) => {
                let mut json_encoded_secret = String::new();
                if let Err(io_err) = f.read_to_string(&mut json_encoded_secret) {
                    return secret_io_error(io_err)
                }
                match json::decode::<ConsoleApplicationSecret>(&json_encoded_secret) {
                    Err(json_decode_error) => return Err(ArgumentError::SecretDecoderError(
                                                (secret_str(), json_decode_error)
                                              )),
                    Ok(console_secret) => match console_secret.installed {
                        Some(secret) => return Ok(secret),
                        None => return Err(ArgumentError::SecretFormatError(secret_str()))
                    },
                }
            }
        }
    }
    unreachable!();
}