//! Definitions of standardized error payloads that cover information needed to describe
//! common failure cases.
//!
//! Services can create and expose their own error payloads when required to sufficiently describe
//! the domain, but should generally default to using ones defined here.

// Attribution: *Heavily* inspired by the Google Cloud API Error Model
//  https://github.com/googleapis/googleapis/blob/master/google/rpc/error_details.proto

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorInfo<M> {
    /// The reason of the error. This is a constant value that identifies the proximate cause of
    /// the error. Error reasons are unique within a particular domain of errors. This should be at
    /// most 63 characters and match a regular expression of `[A-Z][A-Z0-9_]+[A-Z0-9]`, which
    /// represents UPPER_SNAKE_CASE.
    reason: String,

    /// The logical grouping to which the "reason" belongs.
    ///
    /// The error domain is typically the registered service name of the tool or product that
    /// generates the error.
    domain: String,

    // TODO: generic, or opinionated?
    /// Additional structured details about this error.
    ///
    /// Keys should match /[a-zA-Z0-9-_]/ and be limited to 64 characters in length. When
    /// identifying the current value of an exceeded limit, the units should be contained in the
    /// key, not the value.  For example, rather than {"instanceLimit": "100/request"}, should be
    /// returned as, {"instanceLimitPerRequest": "100"}, if the client exceeds the number of
    /// instances that can be created in a single (batch) request.
    metadata: HashMap<String, M>,
}

impl<M> ErrorInfo<M> {
    pub fn new(reason: String, domain: String, metadata: HashMap<String, M>) -> Self {
        Self {
            reason,
            domain,
            metadata,
        }
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn domain(&self) -> &str {
        &self.domain
    }

    pub fn metadata(&self) -> &HashMap<String, M> {
        &self.metadata
    }
}
