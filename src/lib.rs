use std::{
    collections::{BTreeSet, HashSet},
    convert::Infallible,
    fmt,
};

use jayson::{DeserializeFromValue, ValueKind, ValuePointerRef};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, DeserializeFromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[jayson(rename_all = camelCase, deny_unknown_fields)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub offset: usize,
    pub limit: usize,
    pub page: Option<usize>,
    pub hits_per_page: Option<usize>,
    pub attributes_to_retrieve: Option<BTreeSet<String>>,
    pub attributes_to_crop: Option<Vec<String>>,
    pub crop_length: usize,
    pub attributes_to_highlight: Option<HashSet<String>>,
    // Default to false
    pub show_matches_position: bool,
    pub filter: Option<Value>,
    pub sort: Option<Vec<String>>,
    pub facets: Option<Vec<String>>,
    pub highlight_pre_tag: String,
    pub highlight_post_tag: String,
    pub crop_marker: String,
    pub matching_strategy: String,
}

#[derive(Debug)]
pub struct MeiliDeserError(String);

impl std::fmt::Display for MeiliDeserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for MeiliDeserError {}

impl jayson::MergeWithError<Infallible> for MeiliDeserError {
    fn merge(
        _self_: Option<Self>,
        _other: Infallible,
        _merge_location: jayson::ValuePointerRef,
    ) -> Result<Self, Self> {
        unreachable!()
    }
}

impl jayson::MergeWithError<MeiliDeserError> for MeiliDeserError {
    fn merge(
        _self_: Option<Self>,
        other: MeiliDeserError,
        _merge_location: ValuePointerRef,
    ) -> Result<Self, Self> {
        Err(other)
    }
}
impl jayson::DeserializeError for MeiliDeserError {
    /// Return the origin of the error, if it can be found
    fn location(&self) -> Option<jayson::ValuePointer> {
        None
    }
    /// Create a new error due to an unexpected value kind.
    ///
    /// Return `Ok` to continue deserializing or `Err` to fail early.
    fn incorrect_value_kind(
        _self_: Option<Self>,
        actual: ValueKind,
        accepted: &[ValueKind],
        location: ValuePointerRef,
    ) -> Result<Self, Self> {
        Err(MeiliDeserError(format!(
            "incorrect value kind expected one of {:?} but got {actual} at {}",
            accepted,
            location.to_owned()
        )))
    }
    /// Create a new error due to a missing key.
    ///
    /// Return `Ok` to continue deserializing or `Err` to fail early.
    fn missing_field(
        _self_: Option<Self>,
        field: &str,
        _location: ValuePointerRef,
    ) -> Result<Self, Self> {
        Err(MeiliDeserError(format!("missing field {field}")))
    }
    /// Create a new error due to finding an unknown key.
    ///
    /// Return `Ok` to continue deserializing or `Err` to fail early.
    fn unknown_key(
        _self_: Option<Self>,
        key: &str,
        _accepted: &[&str],
        _location: ValuePointerRef,
    ) -> Result<Self, Self> {
        Err(MeiliDeserError(format!("unknown key {key}")))
    }
    /// Create a new error with the custom message.
    ///
    /// Return `Ok` to continue deserializing or `Err` to fail early.
    fn unexpected(
        _self_: Option<Self>,
        msg: &str,
        _location: ValuePointerRef,
    ) -> Result<Self, Self> {
        Err(MeiliDeserError(format!("unexpected {msg}")))
    }
}
