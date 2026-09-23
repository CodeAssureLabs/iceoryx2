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
use iceoryx2::service::strip_internal_prefix;

#[test]
fn naming_prefix_strip_internal_prefix_removes_prefix() {
    let internal_service_name = ServiceName::new("iox2://My/Service").expect("valid service name");
    let result = strip_internal_prefix(&internal_service_name);
    assert_eq!(result, "My/Service");
}

#[test]
fn naming_prefix_strip_internal_prefix_leaves_unprefixed_unchanged() {
    let service_name = ServiceName::new("My/Service").expect("valid service name");
    let result = strip_internal_prefix(&service_name);
    assert_eq!(result, "My/Service");
}
