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

//! Additional sanity checks for [`NodeName`]s before they are stored in the
//! global management segment.

use iceoryx2_log::fatal_panic;

use crate::node::node_name::NodeName;

/// Aborts when `name` is empty; an empty node name can never be resolved by the
/// discovery service.
#[allow(dead_code)]
pub(crate) fn assert_valid_node_name(name: &NodeName) {
    if NodeName::as_str(name).is_empty() {
        fatal_panic!(from "assert_valid_node_name",
            "The node name must not be empty.");
    }
}
