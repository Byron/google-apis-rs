pub mod duration {
    use serde::{Deserialize, Deserializer};
    use serde_with::{DeserializeAs, SerializeAs};
    use std::fmt::Formatter;
    use std::str::FromStr;

    use chrono::Duration;

    const MAX_SECONDS: i64 = 315576000000i64;

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

    fn duration_from_str(s: &str) -> Result<Duration, ParseDurationError> {
        // TODO: Test strings like -.s, -0.0s
        let value = match s.strip_suffix('s') {
            None => return Err(ParseDurationError::MissingSecondSuffix),
            Some(v) => v,
        };

        let (seconds, nanoseconds) = if let Some((seconds, nanos)) = value.split_once('.') {
            let is_neg = seconds.starts_with('-');
            let seconds = i64::from_str(seconds)?;
            let nano_magnitude = nanos.chars().filter(|c| c.is_ascii_digit()).count() as u32;
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

        if seconds >= MAX_SECONDS {
            Err(ParseDurationError::SecondOverflow {
                seconds,
                max_seconds: MAX_SECONDS,
            })
        } else if seconds <= -MAX_SECONDS {
            Err(ParseDurationError::SecondUnderflow {
                seconds,
                min_seconds: -MAX_SECONDS,
            })
        } else {
            Ok(Duration::seconds(seconds) + Duration::nanoseconds(nanoseconds.into()))
        }
    }

    pub fn to_string(duration: &Duration) -> String {
        let seconds = duration.num_seconds();
        let nanoseconds = (*duration - Duration::seconds(seconds))
            .num_nanoseconds()
            .expect("absolute number of nanoseconds is less than 1 billion")
            as i32;
        if nanoseconds != 0 {
            if seconds == 0 && nanoseconds.is_negative() {
                format!("-0.{:0>9}s", nanoseconds.abs())
            } else {
                format!("{}.{:0>9}s", seconds, nanoseconds.abs())
            }
        } else {
            format!("{}s", seconds)
        }
    }

    pub struct Wrapper;

    impl SerializeAs<Duration> for Wrapper {
        fn serialize_as<S>(value: &Duration, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            s.serialize_str(&to_string(value))
        }
    }

    impl<'de> DeserializeAs<'de, Duration> for Wrapper {
        fn deserialize_as<D>(deserializer: D) -> Result<Duration, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = Deserialize::deserialize(deserializer)?;
            duration_from_str(s).map_err(serde::de::Error::custom)
        }
    }
}

pub mod standard_base64 {
    use serde::{Deserialize, Deserializer, Serializer};
    use serde_with::{DeserializeAs, SerializeAs};

    pub struct Wrapper;

    pub fn to_string(bytes: &Vec<u8>) -> String {
        base64::encode_config(bytes, base64::STANDARD)
    }

    impl SerializeAs<Vec<u8>> for Wrapper {
        fn serialize_as<S>(value: &Vec<u8>, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            s.serialize_str(&to_string(value))
        }
    }

    impl<'de> DeserializeAs<'de, Vec<u8>> for Wrapper {
        fn deserialize_as<D>(deserializer: D) -> Result<Vec<u8>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s: &str = Deserialize::deserialize(deserializer)?;
            base64::decode_config(s, base64::STANDARD).map_err(serde::de::Error::custom)
        }
    }
}

pub mod urlsafe_base64 {
    use serde::{Deserialize, Deserializer, Serializer};
    use serde_with::{DeserializeAs, SerializeAs};

    pub struct Wrapper;

    pub fn to_string(bytes: &Vec<u8>) -> String {
        base64::encode_config(bytes, base64::URL_SAFE)
    }

    impl SerializeAs<Vec<u8>> for Wrapper {
        fn serialize_as<S>(value: &Vec<u8>, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            s.serialize_str(&to_string(value))
        }
    }

    impl<'de> DeserializeAs<'de, Vec<u8>> for Wrapper {
        fn deserialize_as<D>(deserializer: D) -> Result<Vec<u8>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s: &str = Deserialize::deserialize(deserializer)?;
            base64::decode_config(s, base64::URL_SAFE).map_err(serde::de::Error::custom)
        }
    }
}

pub fn datetime_to_string(datetime: &chrono::DateTime<chrono::offset::Utc>) -> String {
    datetime.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

#[cfg(test)]
mod test {
    use super::{duration, urlsafe_base64, standard_base64};
    use serde::{Deserialize, Serialize};
    use serde_with::{serde_as, DisplayFromStr};

    #[serde_as]
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct DurationWrapper {
        #[serde_as(as = "Option<duration::Wrapper>")]
        duration: Option<chrono::Duration>,
    }

    #[serde_as]
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Base64URLSafeWrapper {
        #[serde_as(as = "Option<urlsafe_base64::Wrapper>")]
        bytes: Option<Vec<u8>>,
    }

    #[serde_as]
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Base64StandardWrapper {
        #[serde_as(as = "Option<standard_base64::Wrapper>")]
        bytes: Option<Vec<u8>>,
    }

    #[serde_as]
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct I64Wrapper {
        #[serde_as(as = "Option<DisplayFromStr>")]
        num: Option<i64>,
    }

    #[test]
    fn test_duration_de_success_cases() {
        let durations = [
            ("-0.2s", -200_000_000),
            ("0.000000001s", 1),
            ("999.999999999s", 999_999_999_999),
            ("129s", 129_000_000_000),
            ("0.123456789s", 123_456_789),
        ];
        for (repr, nanos) in durations.into_iter() {
            let wrapper: DurationWrapper =
                serde_json::from_str(&format!("{{\"duration\": \"{}\"}}", repr)).unwrap();
            assert_eq!(
                Some(nanos),
                wrapper.duration.unwrap().num_nanoseconds(),
                "parsed \"{}\" expecting Duration with {}ns",
                repr,
                nanos
            );
        }
    }

    #[test]
    fn test_duration_de_failure_cases() {
        let durations = ["1.-3s", "1.1111111111s", "1.2"];
        for repr in durations.into_iter() {
            assert!(
                serde_json::from_str::<DurationWrapper>(&format!("{{\"duration\": \"{}\"}}", repr))
                    .is_err(),
                "parsed \"{}\" expecting err",
                repr
            );
        }
    }

    #[test]
    fn test_duration_ser_success_cases() {
        let durations = [
            -200_000_000,
            1,
            999_999_999_999,
            129_000_000_000,
            123_456_789,
        ];

        for nanos in durations.into_iter() {
            let wrapper = DurationWrapper {
                duration: Some(chrono::Duration::nanoseconds(nanos)),
            };
            let s = serde_json::to_string(&wrapper);
            assert!(s.is_ok(), "Could not serialize {}ns", nanos);
            let s = s.unwrap();
            assert_eq!(
                wrapper,
                serde_json::from_str(&s).unwrap(),
                "round trip should return same duration"
            );
        }
    }

    #[test]
    fn standard_base64_de_success_cases() {
        let wrapper: Base64StandardWrapper =
            serde_json::from_str(r#"{"bytes": "cVhabzk6U21uOkN+MylFWFRJMVFLdEh2MShmVHp9"}"#).unwrap();
        assert_eq!(Some(b"qXZo9:Smn:C~3)EXTI1QKtHv1(fTz}".as_slice()), wrapper.bytes.as_deref());
    }

    #[test]
    fn urlsafe_base64_de_success_cases() {
        let wrapper: Base64URLSafeWrapper =
            serde_json::from_str(r#"{"bytes": "aGVsbG8gd29ybGQ="}"#).unwrap();
        assert_eq!(Some(b"hello world".as_slice()), wrapper.bytes.as_deref());
    }

    #[test]
    fn urlsafe_base64_de_failure_cases() {
        assert!(serde_json::from_str::<Base64URLSafeWrapper>(r#"{"bytes": "aGVsbG8gd29ybG+Q"}"#).is_err());
    }

    #[test]
    fn standard_base64_de_failure_cases() {
        assert!(serde_json::from_str::<Base64StandardWrapper>(r#"{"bytes": "%"}"#).is_err());
    }

    #[test]
    fn urlsafe_base64_roundtrip() {
        let wrapper = Base64URLSafeWrapper {
            bytes: Some(b"Hello world!".to_vec()),
        };
        let s = serde_json::to_string(&wrapper).expect("serialization of bytes infallible");
        assert_eq!(wrapper, serde_json::from_str::<Base64URLSafeWrapper>(&s).unwrap());
    }

    #[test]
    fn standard_base64_roundtrip() {
        let wrapper = Base64StandardWrapper {
            bytes: Some(b"Hello world!".to_vec()),
        };
        let s = serde_json::to_string(&wrapper).expect("serialization of bytes infallible");
        assert_eq!(wrapper, serde_json::from_str::<Base64StandardWrapper>(&s).unwrap());
    }

    #[test]
    fn num_roundtrip() {
        let wrapper = I64Wrapper {
            num: Some(i64::MAX),
        };

        let json_repr = &serde_json::to_string(&wrapper);
        assert!(json_repr.is_ok(), "serialization should succeed");
        assert_eq!(
            wrapper,
            serde_json::from_str(&format!("{{\"num\": \"{}\"}}", i64::MAX)).unwrap()
        );
        assert_eq!(
            wrapper,
            serde_json::from_str(json_repr.as_ref().unwrap()).unwrap(),
            "round trip should succeed"
        );
    }

    #[test]
    fn test_empty_wrapper() {
        assert_eq!(
            DurationWrapper { duration: None },
            serde_json::from_str("{}").unwrap()
        );
        assert_eq!(
            Base64URLSafeWrapper { bytes: None },
            serde_json::from_str("{}").unwrap()
        );
        assert_eq!(
            Base64StandardWrapper { bytes: None },
            serde_json::from_str("{}").unwrap()
        );

        assert_eq!(
            I64Wrapper { num: None },
            serde_json::from_str("{}").unwrap()
        );
    }
}
