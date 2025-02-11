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
    /// Endpoint to perform the request for
    ///
    /// E.g. `format!("/users/{}", user_id)`
    fn endpoint(&self) -> String;

    /// HTTP method to use
    fn method(&self) -> Method;

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

    /// Username and password
    #[inline]
    fn basic_auth(&self) -> Option<(String, Option<String>)> {
        None
    }

    /// The body of the request
    ///
    /// Returns `Some(self)` by default
    ///
    /// Exists so you can alter the body
    /// or skip it entirely
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

    /// Build and execute the request
    fn send(&self, client: &Client, base_url: &str) -> impl Future<Output = Result<T, Error>> {
        let request = self.build(client, base_url);

        async move { self.exec(request).await }
    }

    /// Execute the request and deserialize
    /// the response into `T`
    ///
    /// Can be used to pass your custom builder
    /// while still utilizing the built-in
    /// parsing and type-mapping
    fn exec(&self, builder: RequestBuilder) -> impl Future<Output = Result<T, Error>> {
        async move {
            let response = builder.send().await.map_err(|_| Error::ClientError)?;

            Ok(self.from_response(response).await?)
        }
    }

    /// Execute the request and deserialize
    /// the response into `Option<T>`
    ///
    /// Can be used to pass your custom builder
    /// while still utilizing the built-in
    /// parsing and type-mapping
    fn exec_opt(&self, builder: RequestBuilder) -> impl Future<Output = Result<Option<T>, Error>> {
        async move {
            let response = builder.send().await.map_err(|_| Error::ClientError)?;

            Ok(self.from_response_opt(response).await?)
        }
    }

    /// Deserialize `reqwest::Response` into `T`
    fn from_response(&self, response: Response) -> impl Future<Output = Result<T, Error>> {
        async move {
            Ok(self
                .check_response(response)
                .await?
                .json::<T>()
                .await
                .map_err(|inner| Error::ClientDecodeError(inner.to_string()))?)
        }
    }

    /// Deserialize `reqwest::Response` into `Option<T>`
    fn from_response_opt(
        &self,
        response: Response,
    ) -> impl Future<Output = Result<Option<T>, Error>> {
        async move { Ok(self.check_response(response).await?.json::<T>().await.ok()) }
    }

    /// Deserialize `reqwest::Response` into
    /// `Error::ResponseError` if the response
    /// was erroneous
    fn check_response(&self, response: Response) -> impl Future<Output = Result<Response, Error>> {
        async move {
            if let Err(_) = response.error_for_status_ref() {
                return Err(Error::ResponseError((
                    response.status(),
                    response.json::<Value>().await.ok(),
                ))
                .into());
            }

            Ok(response)
        }
    }
}
