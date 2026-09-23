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

use iceoryx2_bb_system_types::hashed_file_name::hashed_file_name;
use iceoryx2_bb_testing::assert_that;
use iceoryx2_bb_testing_macros::test;

#[test]
pub fn hashed_file_name_is_deterministic() {
    let lhs = hashed_file_name(b"some arbitrary long identifier");
    let rhs = hashed_file_name(b"some arbitrary long identifier");

    assert_that!(lhs, eq rhs);
}

#[test]
pub fn hashed_file_name_differs_for_different_input() {
    let lhs = hashed_file_name(b"first identifier");
    let rhs = hashed_file_name(b"second identifier");

    assert_that!(lhs, ne rhs);
}
