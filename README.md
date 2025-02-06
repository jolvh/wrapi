# Wrapi

**Status**: 🚧 Work in Progress / Experimental

This project is currently under development and is considered experimental. The API may change at any time.

## Introduction

This library provides a trait-based approach to wrapping and interacting with HTTP APIs using `reqwest` and `serde`.

It comes with a built-in `Request` trait that can be used to define a request for an API endpoint. This trait provides a convenient way to define the request's method, endpoint, headers, query parameters, form parameters, and body.

Requests are not tied to a client instance, allowing you to bring your own.
