// Copyright (c) 2025 Contributors to the Eclipse Foundation
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

//! Probe module: bounces a counter back to [`crate::gtprobe_ping`].

use crate::gtprobe_ping::ping;

/// Decrements `remaining` and hands the turn back to [`ping`] until it reaches zero.
pub fn pong(remaining: u32) -> u32 {
    if remaining == 0 {
        return 0;
    }
    ping(remaining - 1) + 1
}
