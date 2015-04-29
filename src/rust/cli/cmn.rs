use oauth2::{ApplicationSecret, ConsoleApplicationSecret, TokenStorage, Token};
use rustc_serialize::json;
use mime::Mime;
use clap::{App, SubCommand};

use std::fs;
use std::env;
use std::io;
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::string::ToString;
use std::io::{Write, Read, stdout};

use std::default::Default;

const FIELD_SEP: char = '.';


fn make_subcommand(command_name: &str, desc: Option<&str>,
                                       args: &Vec<(Option<&str>, Option<&str>, Option<&str>, 
                                                   Option<bool>, Option<bool>)>)
                                                   -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
   // arg_name: Option<&str>,
   // short_name: Option<&str>,
   // help: Option<&str>,
   // % if flag is not None:
   //                      .takes_value(${rust_boolean(arg_name)})
   // required: Option<bool>,
   // multiple: Option<bool>
    SubCommand::new(command_name)
}


#[derive(Clone, Default)]
pub struct FieldCursor(Vec<String>);

impl ToString for  FieldCursor {
    fn to_string(&self) -> String {
        self.0.connect(".")
    }
}

impl FieldCursor {
    pub fn set(&mut self, value: &str) -> Result<(), CLIError> {
        if value.len() == 0 {
            return Err(CLIError::Field(FieldError::Empty))
        }

        let mut first_is_field_sep = false;
        let mut char_count: usize = 0;
        let mut last_c = FIELD_SEP;
        let mut num_conscutive_field_seps = 0;

        let mut field = String::new();
        let mut fields = self.0.clone();

        let push_field = |fs: &mut Vec<String>, f: &mut String| {
            if f.len() > 0 {
                fs.push(f.clone());
                f.truncate(0);
            }
        };

        for (cid, c) in value.chars().enumerate() {
            char_count += 1;

            if c == FIELD_SEP {
                if cid == 0 {
                    first_is_field_sep = true;
                }
                num_conscutive_field_seps += 1;
                if cid > 0 && last_c == FIELD_SEP {
                    if fields.pop().is_none() {
                        return Err(CLIError::Field(FieldError::PopOnEmpty(value.to_string())))
                    }
                } else {
                    push_field(&mut fields, &mut field);
                }
            } else {
                num_conscutive_field_seps = 0;
                if cid == 1 {
                    if first_is_field_sep {
                        fields.truncate(0);
                    }
                }
                field.push(c);
            }

            last_c = c;
        }

        push_field(&mut fields, &mut field);

        if char_count == 1 && first_is_field_sep {
            fields.truncate(0);
        }
        if char_count > 1 && num_conscutive_field_seps == 1 {
            return Err(CLIError::Field(FieldError::TrailingFieldSep(value.to_string())))
        }

        self.0 = fields;
        Ok(())
    }

    pub fn num_fields(&self) -> usize {
        self.0.len()
    }
}

pub fn parse_kv_arg<'a>(kv: &'a str, err: &mut InvalidOptionsError, for_hashmap: bool)
                                                        -> (&'a str, Option<&'a str>) {
    let mut add_err = || err.issues.push(CLIError::InvalidKeyValueSyntax(kv.to_string(),for_hashmap));
    match kv.find('=') {
        None => {
            add_err();
            return (kv, None)
        },
        Some(pos) => {
            let key = &kv[..pos];
            if kv.len() <= pos + 1 {
                add_err();
                return (key, Some(""))
            }
            (key, Some(&kv[pos+1..]))
        }
    }
}

pub fn input_file_from_opts(file_path: &str, err: &mut InvalidOptionsError) -> Option<fs::File> {
    match fs::File::open(file_path) {
        Ok(f) => Some(f),
        Err(io_err) => {
            err.issues.push(CLIError::Input(InputError::IOError((file_path.to_string(), io_err))));
            None
        }
    }
}

pub fn input_mime_from_opts(mime: &str, err: &mut InvalidOptionsError) -> Option<Mime> {
    match mime.parse() {
        Ok(m) => Some(m),
        Err(_) => {
            err.issues.push(CLIError::Input(InputError::Mime(mime.to_string())));
            None
        }
    }
}

// May panic if we can't open the file - this is anticipated, we can't currently communicate this 
// kind of error: TODO: fix this architecture :)
pub fn writer_from_opts(flag: bool, arg: &str) -> Box<Write> {
    if !flag || arg == "-" {
        Box::new(stdout())
    } else {
        Box::new(fs::OpenOptions::new().create(true).write(true).open(arg).unwrap())
    }
}


pub fn arg_from_str<T>(arg: &str, err: &mut InvalidOptionsError, 
                                  arg_name: &'static str, 
                                  arg_type: &'static str) -> T
                                                        where   T: FromStr + Default,
                                                             <T as FromStr>::Err: fmt::Display {
    match FromStr::from_str(arg) {
        Err(perr) => {
            err.issues.push(
                CLIError::ParseError((arg_name, arg_type, arg.to_string(), format!("{}", perr)))
            );
            Default::default()
        },
        Ok(v) => v,
    }
}

pub struct JsonTokenStorage {
    pub program_name: &'static str,
    pub db_dir: String,
}

impl JsonTokenStorage {
    fn path(&self, scope_hash: u64) -> PathBuf {
        Path::new(&self.db_dir).join(&format!("{}-token-{}.json", self.program_name, scope_hash))
    }
}

impl TokenStorage for JsonTokenStorage {
    type Error = io::Error;

    // NOTE: logging might be interesting, currently we swallow all errors
    fn set(&mut self, scope_hash: u64, _: &Vec<&str>, token: Option<Token>) -> Option<io::Error> {
        match token {
            None => {
                match fs::remove_file(self.path(scope_hash)) {
                    Err(err) => 
                        match err.kind() {
                            io::ErrorKind::NotFound => None,
                            _ => Some(err)
                        },
                    Ok(_) => None
                }
            }
            Some(token) => {
                let json_token = json::encode(&token).unwrap();
                match fs::OpenOptions::new().create(true).write(true).open(&self.path(scope_hash)) {
                    Ok(mut f) => {
                        match f.write(json_token.as_bytes()) {
                            Ok(_) => None,
                            Err(io_err) => Some(io_err),
                        }
                    },
                    Err(io_err) => Some(io_err)
                }
            }
        }
    }

    fn get(&self, scope_hash: u64, _: &Vec<&str>) -> Result<Option<Token>, io::Error> {
        match fs::File::open(&self.path(scope_hash)) {
            Ok(mut f) => {
                let mut json_string = String::new();
                match f.read_to_string(&mut json_string) {
                    Ok(_) => Ok(Some(json::decode::<Token>(&json_string).unwrap())),
                    Err(io_err) => Err(io_err),
                }
            },
            Err(io_err) => {
                match io_err.kind() {
                    io::ErrorKind::NotFound => Ok(None),
                    _ => Err(io_err)
                }
            }
        }
    }
}


#[derive(Debug)]
pub enum ApplicationSecretError {
    DecoderError((String, json::DecoderError)),
    FormatError(String),    
}

impl fmt::Display for ApplicationSecretError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ApplicationSecretError::DecoderError((ref path, ref err))
                => writeln!(f, "Could not decode file at '{}' with error: {}", 
                            path, err),
            ApplicationSecretError::FormatError(ref path)
                => writeln!(f, "'installed' field is unset in secret file at '{}'", 
                            path),
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
pub enum InputError {
    IOError((String, io::Error)),
    Mime(String),
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            InputError::IOError((ref file_path, ref io_err))
                => writeln!(f, "Failed to open '{}' for reading with error: {}", file_path, io_err),
            InputError::Mime(ref mime)
                => writeln!(f, "'{}' is not a known mime-type", mime),
        }
    }
}

#[derive(Debug)]
pub enum FieldError {
    PopOnEmpty(String),
    TrailingFieldSep(String),
    Unknown(String),
    Empty,
}


impl fmt::Display for FieldError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            FieldError::PopOnEmpty(ref field)
                => writeln!(f, "'{}': Cannot move up on empty field cursor", field),
            FieldError::TrailingFieldSep(ref field)
                => writeln!(f, "'{}': Single field separator may not be last character", field),
            FieldError::Unknown(ref field)
                => writeln!(f, "Field '{}' does not exist", field),
            FieldError::Empty
                => writeln!(f, "Field names must not be empty"),
        }
    }
}


#[derive(Debug)]
pub enum CLIError {
    Configuration(ConfigurationError),
    ParseError((&'static str, &'static str, String, String)),
    UnknownParameter(String),
    InvalidKeyValueSyntax(String, bool),
    Input(InputError),
    Field(FieldError),
}

impl fmt::Display for CLIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            CLIError::Configuration(ref err) => write!(f, "Configuration -> {}", err),
            CLIError::Input(ref err) => write!(f, "Input -> {}", err),
            CLIError::Field(ref err) => write!(f, "Field -> {}", err),
            CLIError::ParseError((arg_name, type_name, ref value, ref err_desc)) 
                => writeln!(f, "Failed to parse argument '{}' with value '{}' as {} with error: {}",
                            arg_name, value, type_name, err_desc),
            CLIError::UnknownParameter(ref param_name) 
                => writeln!(f, "Parameter '{}' is unknown.", param_name),
            CLIError::InvalidKeyValueSyntax(ref kv, is_hashmap) => {
                let hashmap_info = if is_hashmap { "hashmap " } else { "" };
                writeln!(f, "'{}' does not match {}pattern <key>=<value>", kv, hashmap_info)
            },
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

    pub fn new() -> InvalidOptionsError {
        InvalidOptionsError {
            issues: Vec::new(),
            exit_code: 1,
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

pub fn application_secret_from_directory(dir: &str, 
                                         secret_basename: &str, 
                                         json_app_secret: &str)
                                                        -> Result<ApplicationSecret, CLIError> {
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

                    err = match fs::OpenOptions::new().create(true).write(true).open(&secret_path) {
                        Err(cfe) => cfe,
                        Ok(mut f) => {
                            match f.write(json_app_secret.as_bytes()) {
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