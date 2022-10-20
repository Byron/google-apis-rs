use std::borrow::Cow;

use ::url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
use ::url::Url;

pub struct Params<'a> {
    params: Vec<(&'a str, Cow<'a, str>)>,
}

impl<'a> Params<'a> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            params: Vec::with_capacity(capacity),
        }
    }

    pub fn push<I: Into<Cow<'a, str>>>(&mut self, param: &'a str, value: I) {
        self.params.push((param, value.into()))
    }

    pub fn extend<I: Iterator<Item = (&'a String, IC)>, IC: Into<Cow<'a, str>>>(
        &mut self,
        params: I,
    ) {
        self.params
            .extend(params.map(|(k, v)| (k.as_str(), v.into())))
    }

    pub fn get(&self, param_name: &str) -> Option<&str> {
        self.params
            .iter()
            .find(|(name, _)| name == &param_name)
            .map(|(_, param)| param.as_ref())
    }

    pub fn uri_replacement(
        &self,
        url: String,
        param: &str,
        from: &str,
        url_encode: bool,
    ) -> String {
        if url_encode {
            let mut replace_with: Cow<str> = self.get(param).unwrap_or_default().into();
            if from.as_bytes()[1] == b'+' {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET)
                    .to_string()
                    .into();
            }
            url.replace(from, &replace_with)
        } else {
            let replace_with = self
                .get(param)
                .expect("to find substitution value in params");

            url.replace(from, replace_with)
        }
    }

    pub fn remove_params(&mut self, to_remove: &[&str]) {
        self.params.retain(|(n, _)| !to_remove.contains(n))
    }

    pub fn inner_mut(&mut self) -> &mut Vec<(&'a str, Cow<'a, str>)> {
        self.params.as_mut()
    }

    pub fn parse_with_url(&self, url: &str) -> Url {
        Url::parse_with_params(url, &self.params).unwrap()
    }
}
