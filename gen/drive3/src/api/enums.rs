use super::*;



#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The source of files to list. Deprecated: use 'corpora' instead.
pub enum FileCorpusEnum {
    

    /// Files shared to the user's domain.
    ///
    /// "domain"
    #[serde(rename="domain")]
    Domain,
    

    /// Files owned by or shared to the user. If a user has permissions on a Shared Drive, the files inside it won't be retrieved unless the user has created, opened, or shared the file.
    ///
    /// "user"
    #[serde(rename="user")]
    User,
}

impl AsRef<str> for FileCorpusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FileCorpusEnum::Domain => "user",
            FileCorpusEnum::User => "user",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FileCorpusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


