from random import choice, randint, random, seed

from .rust_type import Base, HashMap, Vec

seed(1337)

WORDS = [
    w.strip(",")
    for w in "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.".split(
        " "
    )
]


def chrono_date(y=None, m=None, d=None):
    y = randint(1, 9999) if y is None else y
    m = randint(1, 12) if m is None else m
    d = randint(1, 31) if d is None else d
    return f"chrono::NaiveDate::from_ymd({y}, {m}, {d})"


CHRONO_DATE = "chrono::NaiveDate"
CHRONO_DATETIME = "chrono::DateTime<chrono::offset::Utc>"
CHRONO_UTC_NOW = "chrono::Utc::now()"
USE_FORMAT = "use_format_field"

RUST_TYPE_MAP = {
    "boolean": Base("bool"),
    "integer": USE_FORMAT,
    "number": USE_FORMAT,
    "uint32": Base("u32"),
    "double": Base("f64"),
    "float": Base("f32"),
    "int32": Base("i32"),
    "any": Base("serde_json::Value"),
    "int64": Base("i64"),
    "uint64": Base("u64"),
    "array": Vec(None),
    "string": Base("String"),
    "object": HashMap(None, None),
    # https://github.com/protocolbuffers/protobuf/blob/ec1a70913e5793a7d0a7b5fbf7e0e4f75409dd41/src/google/protobuf/timestamp.proto
    # In JSON format, the Timestamp type is encoded as a string in the [RFC 3339] format
    "google-datetime": Base(CHRONO_DATETIME),
    # Per .json files: RFC 3339 timestamp
    "date-time": Base(CHRONO_DATETIME),
    # Per .json files: A date in RFC 3339 format with only the date part
    # e.g. "2013-01-15"
    "date": Base(CHRONO_DATE),
    # https://github.com/protocolbuffers/protobuf/blob/ec1a70913e5793a7d0a7b5fbf7e0e4f75409dd41/src/google/protobuf/duration.proto
    "google-duration": Base(f"chrono::Duration"),
    # guessing bytes is universally url-safe b64
    "byte": Vec(Base("u8")),
    # https://github.com/protocolbuffers/protobuf/blob/ec1a70913e5793a7d0a7b5fbf7e0e4f75409dd41/src/google/protobuf/field_mask.proto
    "google-fieldmask": Base("common::FieldMask"),
}

RUST_TYPE_RND_MAP = {
    "bool": lambda: str(bool(randint(0, 1))).lower(),
    "u32": lambda: randint(0, 100),
    "u64": lambda: randint(0, 100),
    "f64": lambda: random(),
    "f32": lambda: random(),
    "i32": lambda: randint(-101, -1),
    "i64": lambda: randint(-101, -1),
    "String": lambda: '"%s"' % choice(WORDS),
    "&str": lambda: '"%s"' % choice(WORDS),
    "&Vec<String>": lambda: '&vec!["%s".into()]' % choice(WORDS),
    "Vec<u8>": lambda: f"vec![0, 1, 2, 3]",
    # why a reference to Vec? Because it works. Should be slice, but who knows how typing works here.
    "&Vec<u8>": lambda: f"&vec![0, 1, 2, 3]",
    # TODO: styling this
    f"chrono::Duration": lambda: f"chrono::Duration::seconds({randint(0, 9999999)})",
    CHRONO_DATE: chrono_date,
    CHRONO_DATETIME: lambda: CHRONO_UTC_NOW,
    "FieldMask": lambda: "FieldMask::new::<&str>(&[])",
}

JSON_TO_RUST_DEFAULT = {
    "boolean": "false",
    "uint32": "0",
    "uint64": "0",
    "int32": "-0",
    "int64": "-0",
    "float": "0.0",
    "double": "0.0",
    "string": '""',
    "google-datetime": CHRONO_UTC_NOW,
    "date-time": CHRONO_UTC_NOW,
    "date": chrono_date(2000, 1, 1),
    # https://github.com/protocolbuffers/protobuf/blob/ec1a70913e5793a7d0a7b5fbf7e0e4f75409dd41/src/google/protobuf/duration.proto
    "google-duration": "chrono::Duration::seconds(0)",
    # guessing bytes is universally url-safe b64
    "byte": 'b"hello world"',
    # https://github.com/protocolbuffers/protobuf/blob/ec1a70913e5793a7d0a7b5fbf7e0e4f75409dd41/src/google/protobuf/field_mask.proto
    "google-fieldmask": "apis_common::FieldMask::default()",
}


assert set(JSON_TO_RUST_DEFAULT.keys()).issubset(set(RUST_TYPE_MAP.keys()))
