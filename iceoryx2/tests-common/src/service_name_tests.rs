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
use iceoryx2::service::service_name::ServiceName;
use iceoryx2_bb_testing::assert_that;
use iceoryx2_bb_testing_macros::test;

#[test]
fn user_service_name_is_not_internal() {
    let sut = ServiceName::new("my/funky/service").unwrap();
    assert_that!(sut.is_internal(), eq false);
}

#[test]
fn prefixed_service_name_is_internal() {
    let sut = ServiceName::__internal_new_prefixed("discovery").unwrap();
    assert_that!(sut.is_internal(), eq true);
}
