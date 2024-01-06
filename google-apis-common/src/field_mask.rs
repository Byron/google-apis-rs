use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

fn titlecase(source: &str, dest: &mut String) {
    let mut underscore = false;
    for c in source.chars() {
        if c == '_' {
            underscore = true;
        } else if underscore {
            dest.push(c.to_ascii_uppercase());
            underscore = false;
        } else {
            dest.push(c);
        }
    }
}

fn snakecase(source: &str) -> String {
    let mut dest = String::with_capacity(source.len() + 5);
    for c in source.chars() {
        if c.is_ascii_uppercase() {
            dest.push('_');
            dest.push(c.to_ascii_lowercase());
        } else {
            dest.push(c);
        }
    }
    dest
}

/// A `FieldMask` as defined in `https://github.com/protocolbuffers/protobuf/blob/ec1a70913e5793a7d0a7b5fbf7e0e4f75409dd41/src/google/protobuf/field_mask.proto#L180`
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FieldMask(Vec<String>);

impl FieldMask {
    pub fn new<S: AsRef<str>>(values: &[S]) -> Self {
        return Self(values.iter().map(|s| snakecase(s.as_ref())).collect());
    }
}

impl Serialize for FieldMask {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for FieldMask {
    fn deserialize<D>(deserializer: D) -> Result<FieldMask, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        Ok(FieldMask::from_str(s).unwrap())
    }
}

impl FromStr for FieldMask {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut in_quotes = false;
        let mut prev_ind = 0;
        let mut paths = Vec::new();
        for (i, c) in s.chars().enumerate() {
            if c == '`' {
                in_quotes = !in_quotes;
            } else if in_quotes {
                continue;
            } else if c == ',' {
                paths.push(snakecase(&s[prev_ind..i]));
                prev_ind = i + 1;
            }
        }
        paths.push(snakecase(&s[prev_ind..]));
        Ok(FieldMask(paths))
    }
}

impl Display for FieldMask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut repr = String::new();
        for path in &self.0 {
            titlecase(path, &mut repr);
            repr.push(',');
        }
        repr.pop();
        f.write_str(&repr)
    }
}

#[cfg(test)]
mod test {
    use crate::field_mask::FieldMask;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct FieldMaskWrapper {
        fields: Option<FieldMask>,
    }

    #[test]
    fn field_mask_roundtrip() {
        let wrapper = FieldMaskWrapper {
            fields: Some(FieldMask(vec![
                "user.display_name".to_string(),
                "photo".to_string(),
            ])),
        };
        let json_repr = &serde_json::to_string(&wrapper);
        assert!(json_repr.is_ok(), "serialization should succeed");
        assert_eq!(
            wrapper,
            serde_json::from_str(r#"{"fields": "user.displayName,photo"}"#).unwrap()
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
            FieldMaskWrapper { fields: None },
            serde_json::from_str("{}").unwrap()
        );
    }
}
