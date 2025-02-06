use std::{collections::HashMap, future::Future};

use http::{HeaderMap, Method};
use reqwest::{Client, RequestBuilder, Response};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use super::error::Error;

pub trait Request
where
    Self: Serialize + Send + Sync,
{
    type Response: DeserializeOwned + Send + Sync;

    /// Endpoint to perform the request for
    ///
    /// E.g. `auth`
    fn endpoint(&self) -> &str;

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

    /// The body of the request
    ///
    /// Exists so you can skip sending a
    /// body
    #[inline]
    fn body(&self) -> Option<&Self> {
        Some(self)
    }

    /// Sends the request to the API
    ///
    /// Method from `self.method()` and URL
    /// from `self.url()`
    ///
    /// Applies header, query and form parameters
    /// if they exist
    ///
    /// Appends `self` as the JSON body if it
    /// exists
    fn send(
        &self,
        client: &Client,
        base_url: &str,
    ) -> impl Future<Output = Result<Self::Response, Error>> {
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

        // Apply body
        if let Some(body) = self.body() {
            request = request.json(body);
        }

        async move {
            let response = request.send().await.map_err(|_| Error::ClientError)?;

            Ok(self.from_response(response).await?)
        }
    }

    /// Appends `self` as the JSON body if it exists
    /// and executes the request
    ///
    /// Exists so you can build your own request
    /// but still utilize built-in parsing and
    /// type-mapping
    fn execute(
        &self,
        mut builder: RequestBuilder,
    ) -> impl Future<Output = Result<Self::Response, Error>> {
        // Apply body
        if let Some(body) = self.body() {
            builder = builder.json(body);
        }

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
    fn from_response(
        &self,
        response: Response,
    ) -> impl Future<Output = Result<Self::Response, Error>> {
        async move {
            if !response.status().is_success() {
                return Err(Error::ResponseError((
                    response.status(),
                    response.json::<Value>().await.ok(),
                ))
                .into());
            }

            Ok(response
                .json::<Self::Response>()
                .await
                .map_err(|_| Error::ClientDecodeError)?)
        }
    }
}
