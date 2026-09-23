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

use iceoryx2::prelude::*;
use iceoryx2::service::naming_prefix::strip_internal_prefix;
use iceoryx2_bb_testing::assert_that;
use iceoryx2_bb_testing_macros::test;

#[test]
fn strips_the_internal_prefix_when_present() {
    let sut = ServiceName::__internal_new_prefixed("some_service").unwrap();

    assert_that!(strip_internal_prefix(&sut), eq "some_service");
}

#[test]
fn leaves_names_without_the_internal_prefix_unchanged() {
    let sut = ServiceName::new("some_service").unwrap();

    assert_that!(strip_internal_prefix(&sut), eq "some_service");
}
