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

use alloc::format;

use iceoryx2::prelude::*;
use iceoryx2::service::naming_prefix::strip_internal_prefix;
use iceoryx2::service::service_name::INTERNAL_SERVICE_PREFIX;
use iceoryx2_bb_testing::assert_that;
use iceoryx2_bb_testing_macros::test;

#[test]
fn strip_internal_prefix_removes_prefix_from_internal_names() {
    let value = "hypnotoad/internal/service";
    let expected = format!("{INTERNAL_SERVICE_PREFIX}{value}");
    let sut = ServiceName::__internal_new_prefixed(value).unwrap();

    assert_that!(sut, eq expected.as_str());
    assert_that!(strip_internal_prefix(&sut), eq value);
}

#[test]
fn strip_internal_prefix_keeps_names_without_prefix_unchanged() {
    let value = "hypnotoad/public/service";
    let sut = ServiceName::new(value).unwrap();

    assert_that!(strip_internal_prefix(&sut), eq value);
}

#[test]
fn strip_internal_prefix_only_removes_leading_prefix() {
    let value = "hypnotoad/iox2://not/a/prefix";
    let sut = ServiceName::new(value).unwrap();

    assert_that!(strip_internal_prefix(&sut), eq value);
}
