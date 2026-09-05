use std::borrow::Cow;

use ::url::Url;
use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

/// A collection of URL parameters
pub struct Params<'a> {
    params: Vec<(&'a str, Cow<'a, str>)>,
}

impl<'a> Params<'a> {
    /// Creates a new Params instance with the specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            params: Vec::with_capacity(capacity),
        }
    }

    /// Adds a parameter
    pub fn push<I: Into<Cow<'a, str>>>(&mut self, param: &'a str, value: I) {
        self.params.push((param, value.into()))
    }

    /// Extends the parameters with an iterator
    pub fn extend<I: Iterator<Item = (&'a String, IC)>, IC: Into<Cow<'a, str>>>(
        &mut self,
        params: I,
    ) {
        self.params
            .extend(params.map(|(k, v)| (k.as_str(), v.into())))
    }

    /// Gets a parameter by name
    pub fn get(&self, param_name: &str) -> Option<&str> {
        self.params
            .iter()
            .find(|(name, _)| name == &param_name)
            .map(|(_, param)| param.as_ref())
    }

    /// Replaces a placeholder in a URL with a parameter value
    pub fn uri_replacement(
        &self,
        url: String,
        param: &str,
        from: &str,
        _url_encode: bool,
    ) -> String {
        let replace_with = self.get(param).unwrap_or_default();
        let is_plus = from.starts_with("{+");

        let replace_with = if is_plus {
            const PLUS_ENCODE_SET: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'<')
                .add(b'>')
                .add(b'`')
                .add(b'{')
                .add(b'}');
            percent_encode(replace_with.as_bytes(), PLUS_ENCODE_SET).to_string()
        } else {
            const NORMAL_ENCODE_SET: &AsciiSet = &percent_encoding::NON_ALPHANUMERIC
                .remove(b'-')
                .remove(b'_')
                .remove(b'.')
                .remove(b'~');
            percent_encode(replace_with.as_bytes(), NORMAL_ENCODE_SET).to_string()
        };

        url.replace(from, &replace_with)
    }

    /// Removes parameters
    pub fn remove_params(&mut self, to_remove: &[&str]) {
        self.params.retain(|(n, _)| !to_remove.contains(n))
    }

    /// Returns a mutable reference to the inner parameters
    pub fn inner_mut(&mut self) -> &mut Vec<(&'a str, Cow<'a, str>)> {
        self.params.as_mut()
    }

    /// Parses a URL with the parameters
    pub fn parse_with_url(&self, url: &str) -> Url {
        Url::parse_with_params(url, &self.params).unwrap()
    }
}
