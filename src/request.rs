use std::{collections::HashMap, future::Future};

use http::{HeaderMap, Method};
use reqwest::{Client, RequestBuilder, Response};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use super::error::Error;

pub trait Request<T>
where
    Self: Serialize + Send + Sync,
    T: DeserializeOwned + Send + Sync,
{
    // type Response:

    /// Endpoint to perform the request for
    ///
    /// E.g. `auth`
    fn endpoint(&self) -> String;

    /// HTTP method to use
    ///
    /// Defaults to `GET`
    #[inline]
    fn method(&self) -> Method {
        Method::GET
    }

    /// Header parameters to include in the request
    #[inline]
    fn headers(&self) -> Option<HeaderMap> {
        None
    }

    /// Query parameters to include in the request
    #[inline]
    fn query(&self) -> Option<HashMap<String, String>> {
        None
    }

    /// Form parameters to include in the request
    #[inline]
    fn form(&self) -> Option<HashMap<String, String>> {
        None
    }

    /// Bearer token to include in the request
    #[inline]
    fn bearer(&self) -> Option<String> {
        None
    }

    /// Basic auth
    ///
    /// Username and password
    #[inline]
    fn basic_auth(&self) -> Option<(String, Option<String>)> {
        None
    }

    /// The body of the request
    ///
    /// Exists so you can skip sending a
    /// body
    #[inline]
    fn body(&self) -> Option<&Self> {
        Some(self)
    }

    /// Build the request, adding all existing
    /// attributes and parameters to the request
    ///
    /// Exists so you can use the included builder
    /// but also alter a request before executing it
    fn build(&self, client: &Client, base_url: &str) -> RequestBuilder {
        let mut request =
            client.request(self.method(), format!("{}/{}", base_url, self.endpoint()));

        // Apply headers
        if let Some(headers) = self.headers() {
            request = request.headers(headers);
        }

        // Apply query parameters
        if let Some(query) = self.query() {
            request = request.query(&query);
        }

        // Apply form parameters
        if let Some(form) = self.form() {
            request = request.form(&form);
        }

        // Apply bearer token
        if let Some(bearer) = self.bearer() {
            request = request.bearer_auth(bearer);
        }

        // Apply basic auth
        if let Some((username, password)) = self.basic_auth() {
            request = request.basic_auth(username, password);
        }

        // Apply body
        if let Some(body) = self.body() {
            request = request.json(body);
        }

        request
    }

    /// Builds and executes the request
    fn send(&self, client: &Client, base_url: &str) -> impl Future<Output = Result<T, Error>> {
        let request = self.build(client, base_url);

        async move { self.exec(request).await }
    }

    /// For customizing the request builder
    /// while still utilizing the built-in
    /// parsing and type-mapping
    fn exec(&self, builder: RequestBuilder) -> impl Future<Output = Result<T, Error>> {
        async move {
            let response = builder.send().await.map_err(|_| Error::ClientError)?;

            Ok(self.from_response(response).await?)
        }
    }

    /// Deserializes `reqwest::Response` into
    /// `Self::Response` if the response was
    /// successful
    ///
    /// Deserializes into `Error::ResponseError`
    /// if the response was erroneous
    fn from_response(&self, response: Response) -> impl Future<Output = Result<T, Error>> {
        async move {
            if let Err(_) = response.error_for_status_ref() {
                return Err(Error::ResponseError((
                    response.status(),
                    response.json::<Value>().await.ok(),
                ))
                .into());
            }

            Ok(response
                .json::<T>()
                .await
                .map_err(|inner| Error::ClientDecodeError(inner.to_string()))?)
        }
    }
}
