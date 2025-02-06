use std::collections::HashMap;

use http::HeaderMap;

/// Helper struct to for adding
/// parameters to a request
#[derive(Clone, Debug)]
pub struct Parameters {
    pub headers: Option<HeaderMap>,
    pub query: Option<HashMap<String, String>>,
    pub form: Option<HashMap<String, String>>,
}

impl Parameters {
    pub fn new() -> Self {
        Self {
            headers: None,
            query: None,
            form: None,
        }
    }

    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers = Some(headers);
        self
    }

    pub fn query(mut self, query: HashMap<String, String>) -> Self {
        self.query = Some(query);
        self
    }

    pub fn form(mut self, form: HashMap<String, String>) -> Self {
        self.form = Some(form);
        self
    }
}
