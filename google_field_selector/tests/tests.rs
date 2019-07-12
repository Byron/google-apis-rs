#![allow(dead_code)]

use google_field_selector::FieldSelector;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, FieldSelector)]
#[serde(rename_all = "camelCase")]
struct File {
    id: String,
    mime_type: String,
    sharing_user: Option<UserInfo>,
}

#[derive(Deserialize, FieldSelector)]
#[serde(rename_all = "camelCase")]
struct UserInfo {
    me: bool,
    email_address: String,
    user_attrs: HashMap<String, String>,
}

#[test]
fn basic() {
    #[derive(Deserialize, FieldSelector)]
    #[serde(rename_all = "camelCase")]
    struct Response {
        next_page_token: String,
        files: Vec<File>,
    }

    assert_eq!(
        Response::field_selector(),
        "nextPageToken,files(id,mimeType,sharingUser/me,sharingUser/emailAddress,sharingUser/userAttrs)"
    );
}

#[test]
fn generic_with_flatten() {
    #[derive(Deserialize, FieldSelector)]
    #[serde(rename_all = "camelCase")]
    struct Response<T>
    where
        T: FieldSelector,
    {
        next_page_token: String,
        #[serde(flatten)]
        payload: T,
    }

    #[derive(Deserialize, FieldSelector)]
    #[serde(rename_all = "camelCase")]
    struct ListFiles {
        files: Vec<File>,
    }
    assert_eq!(
        Response::<ListFiles>::field_selector(),
        "nextPageToken,files(id,mimeType,sharingUser/me,sharingUser/emailAddress,sharingUser/userAttrs)"
    );
}

#[test]
fn external_types() {
    use chrono::{DateTime, Utc};
    #[derive(Deserialize)]
    struct MyCustomVec<T>(Vec<T>);

    #[derive(Deserialize, FieldSelector)]
    struct File {
        id: String,

        // Specify that DateTime is a leaf node. Don't treat it as a nested
        // struct that we can specify subselections of.
        #[field_selector(leaf)]
        viewed_by_me_time: DateTime<Utc>,
    }

    #[derive(Deserialize, FieldSelector)]
    #[serde(rename_all = "camelCase")]
    struct Response {
        next_page_token: String,

        // Specify that MyCustomVec should be treated as a container holding
        // elements of File.
        #[field_selector(container_of = "File")]
        files: MyCustomVec<File>,
    }

    assert_eq!(
        Response::field_selector(),
        "nextPageToken,files(id,viewed_by_me_time)"
    );
}
