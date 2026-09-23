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

use core::time::Duration;

use alloc::string::String;

pub trait Testing {
    /// Configuration type of the backend under test.
    type BackendConfig: Default;

    /// Backend configuration the conformance tests create gateways with.
    ///
    /// Implemented per backend rather than defaulted here: the backend's
    /// concrete `BackendConfig` type lives above this crate's layer, so only
    /// the implementor may call its `Default::default()`.
    fn backend_config() -> Self::BackendConfig;

    fn sync(_id: String, _timeout: Duration) -> bool {
        true
    }

    /// Polls `f` with an adaptive backoff until it succeeds or `timeout`
    /// elapses. The distinct failure reasons observed are listed in the
    /// error.
    ///
    /// Implemented per backend rather than defaulted here: the adaptive-wait
    /// polling loop lives above this crate's layer, alongside the other test
    /// support the implementor already brings in.
    fn retry<F>(f: F, timeout: Duration) -> Result<(), String>
    where
        F: FnMut() -> Result<(), &'static str>;
}
