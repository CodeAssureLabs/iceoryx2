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

//! Derives a fixed-length, file-system safe [`FileName`] from an arbitrary byte
//! sequence, e.g. to map a long user-provided identifier onto a shared-memory name.

use alloc::string::ToString;

use crate::base64url::Base64Url;
use crate::file_name::FileName;

/// Hashes `bytes` and returns the base64url encoded digest as a [`FileName`].
///
/// **Shall not be used for security critical use cases** since the underlying
/// Sha1 digest is not collision resistant.
pub fn hashed_file_name(bytes: &[u8]) -> FileName {
    let mut hash = sha1_smol::Sha1::new();
    hash.update(bytes);
    // the hex digest is always a valid Base64Url representation
    Base64Url::new(hash.digest().to_string().as_bytes())
        .unwrap()
        .as_file_name()
}
