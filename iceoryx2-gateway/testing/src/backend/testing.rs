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

#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]

use alloc::collections::btree_set::BTreeSet;
use alloc::vec::Vec;
use alloc::{format, string::String};
use core::time::Duration;

use iceoryx2_bb_posix::adaptive_wait::AdaptiveWaitBuilder;

pub struct Testing;

impl iceoryx2_gateway_backend::traits::testing::Testing for Testing {
    type BackendConfig = crate::backend::Config;

    fn backend_config() -> Self::BackendConfig {
        Self::BackendConfig::default()
    }

    fn retry<F>(mut f: F, timeout: Duration) -> Result<(), String>
    where
        F: FnMut() -> Result<(), &'static str>,
    {
        let mut errors = BTreeSet::<&'static str>::new();

        let mut adaptive_wait = AdaptiveWaitBuilder::new()
            .create()
            .expect("failed to create adaptive wait");

        let succeeded = adaptive_wait
            .wait_while_with_timeout(
                || -> Result<bool, ()> {
                    match f() {
                        Ok(()) => Ok(false),
                        Err(failure) => {
                            errors.insert(failure);
                            Ok(true)
                        }
                    }
                },
                timeout,
            )
            .expect("failed to wait");

        if succeeded {
            return Ok(());
        }

        errors.insert("Timeout exceeded.");
        let errors_formatted = errors
            .iter()
            .map(|e| format!("  - {}", e))
            .collect::<Vec<_>>()
            .join("\n");
        Err(errors_formatted)
    }
}
