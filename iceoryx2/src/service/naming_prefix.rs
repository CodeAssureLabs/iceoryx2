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

//! Helpers to inspect the internal `iox2://` prefix of a [`ServiceName`](crate::service::service_name::ServiceName).

use crate::service::service_name::{INTERNAL_SERVICE_PREFIX, ServiceName};

/// Returns the service name without the internal `iox2://` prefix. Names without
/// the prefix are returned unchanged.
pub fn strip_internal_prefix(name: &ServiceName) -> &str {
    name.as_str()
        .strip_prefix(INTERNAL_SERVICE_PREFIX)
        .unwrap_or(name.as_str())
}
