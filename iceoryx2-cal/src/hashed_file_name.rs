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

use crate::hash::Hash;
use crate::hash::sha1::Sha1;
use iceoryx2_bb_system_types::file_name::FileName;

/// Hashes `bytes` and returns the base64url encoded digest as a [`FileName`].
pub fn hashed_file_name(bytes: &[u8]) -> FileName {
    Sha1::new(bytes).value().as_base64url().as_file_name()
}
