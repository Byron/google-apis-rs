pub mod duration {
    use std::fmt::Formatter;
    use std::str::FromStr;

    use serde::{Deserialize, Deserializer, Serializer};

    use chrono::Duration;

    #[derive(Debug)]
    enum ParseDurationError {
        MissingSecondSuffix,
        NanosTooSmall,
        ParseIntError(std::num::ParseIntError),
        SecondOverflow { seconds: i64, max_seconds: i64 },
        SecondUnderflow { seconds: i64, min_seconds: i64 },
    }

    impl From<std::num::ParseIntError> for ParseDurationError {
        fn from(pie: std::num::ParseIntError) -> Self {
            ParseDurationError::ParseIntError(pie)
        }
    }

    impl std::fmt::Display for ParseDurationError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                ParseDurationError::MissingSecondSuffix => write!(f, "'s' suffix was not present"),
                ParseDurationError::NanosTooSmall => {
                    write!(f, "more than 9 digits of second precision required")
                }
                ParseDurationError::ParseIntError(pie) => write!(f, "{:?}", pie),
                ParseDurationError::SecondOverflow {
                    seconds,
                    max_seconds,
                } => write!(
                    f,
                    "seconds overflow (got {}, maximum seconds possible {})",
                    seconds, max_seconds
                ),
                ParseDurationError::SecondUnderflow {
                    seconds,
                    min_seconds,
                } => write!(
                    f,
                    "seconds underflow (got {}, minimum seconds possible {})",
                    seconds, min_seconds
                ),
            }
        }
    }

    impl std::error::Error for ParseDurationError {}

    fn parse_duration(s: &str) -> Result<Duration, ParseDurationError> {
        let abs_duration = 315576000000i64;
        // TODO: Test strings like -.s, -0.0s
        let value = match s.strip_suffix('s') {
            None => return Err(ParseDurationError::MissingSecondSuffix),
            Some(v) => v,
        };

        let (seconds, nanoseconds) = if let Some((seconds, nanos)) = value.split_once('.') {
            let is_neg = seconds.starts_with("-");
            let seconds = i64::from_str(seconds)?;
            let nano_magnitude = nanos.chars().filter(|c| c.is_digit(10)).count() as u32;
            if nano_magnitude > 9 {
                // not enough precision to model the remaining digits
                return Err(ParseDurationError::NanosTooSmall);
            }

            // u32::from_str prevents negative nanos (eg '0.-12s) -> lossless conversion to i32
            // 10_u32.pow(...) scales number to appropriate # of nanoseconds
            let nanos = u32::from_str(nanos)? as i32;

            let mut nanos = nanos * 10_i32.pow(9 - nano_magnitude);
            if is_neg {
                nanos = -nanos;
            }
            (seconds, nanos)
        } else {
            (i64::from_str(value)?, 0)
        };

        if seconds >= abs_duration {
            Err(ParseDurationError::SecondOverflow {
                seconds,
                max_seconds: abs_duration,
            })
        } else if seconds <= -abs_duration {
            Err(ParseDurationError::SecondUnderflow {
                seconds,
                min_seconds: -abs_duration,
            })
        } else {
            Ok(Duration::seconds(seconds) + Duration::nanoseconds(nanoseconds.into()))
        }
    }

    pub fn serialize<S>(x: &Option<Duration>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match x {
            None => s.serialize_none(),
            Some(x) => {
                let seconds = x.num_seconds();
                let nanoseconds = (*x - Duration::seconds(seconds))
                    .num_nanoseconds()
                    .expect("number of nanoseconds is less than or equal to 1 billion")
                    as i32;
                // might be left with -1 + non-zero nanos
                if nanoseconds != 0 {
                    if seconds == 0 && nanoseconds.is_negative() {
                        s.serialize_str(&format!("-0.{}s", nanoseconds.abs()))
                    } else {
                        s.serialize_str(&format!("{}.{}s", seconds, nanoseconds.abs()))
                    }
                } else {
                    s.serialize_str(&format!("{}s", seconds))
                }
            }
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<&str> = Deserialize::deserialize(deserializer)?;
        s.map(|s| parse_duration(s).map_err(serde::de::Error::custom))
            .transpose()
    }
}

pub mod urlsafe_base64 {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(x: &Option<Vec<u8>>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match x {
            None => s.serialize_none(),
            Some(x) => s.serialize_some(&base64::encode_config(x, base64::URL_SAFE)),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<&str> = Deserialize::deserialize(deserializer)?;
        s.map(|s| base64::decode_config(s, base64::URL_SAFE).map_err(serde::de::Error::custom))
            .transpose()
    }
}

// TODO: https://developers.google.com/protocol-buffers/docs/reference/csharp/class/google/protobuf/well-known-types/field-mask
// "google-fieldmask"
