// Copyright (c) 2026 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache Software License 2.0 which is available at
// https://www.apache.org/licenses/LICENSE-2.0, or the MIT license
// which is available at https://opensource.org/licenses/MIT.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Builds the key under which a static storage entry of a service is published.

use iceoryx2_bb_container::string::{StaticString, StringModificationError};

const MAX_SERVICE_NAME_LENGTH: usize = 255;
const INTERNAL_SERVICE_PREFIX: &str = "iox2://";

/// Errors that can occur when creating a service key.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ServiceKeyError {
    /// The service name has invalid content (e.g., empty string).
    InvalidContent,
    /// The service name exceeds the maximum allowed length.
    ExceedsMaximumLength,
}

impl core::fmt::Display for ServiceKeyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ServiceKeyError::{self:?}")
    }
}

impl core::error::Error for ServiceKeyError {}

impl From<StringModificationError> for ServiceKeyError {
    fn from(error: StringModificationError) -> Self {
        match error {
            StringModificationError::InsertWouldExceedCapacity => {
                ServiceKeyError::ExceedsMaximumLength
            }
            StringModificationError::InvalidCharacter => ServiceKeyError::InvalidContent,
        }
    }
}

type ServiceKeyString = StaticString<MAX_SERVICE_NAME_LENGTH>;

/// Validates `name` and returns a static string used as a static storage key.
pub fn service_key(name: &str) -> Result<ServiceKeyString, ServiceKeyError> {
    if name.is_empty() || name.starts_with(INTERNAL_SERVICE_PREFIX) {
        return Err(ServiceKeyError::InvalidContent);
    }

    ServiceKeyString::try_from(name).map_err(ServiceKeyError::from)
}
