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

//! Derives the lookup key under which a service is registered.

use alloc::string::String;

use iceoryx2_cal::hash::{Hash, sha1::Sha1};

/// Returns the base64url encoded lookup key of the service `name`.
pub fn service_lookup_key(name: &str) -> String {
    Sha1::new(name.as_bytes()).value().into()
}
