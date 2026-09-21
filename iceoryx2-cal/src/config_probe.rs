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

//! Probes whether the host satisfies the POSIX requirements of the communication layer.

use iceoryx2_bb_posix::config::{
    ComplianceCheckMode, does_system_satisfy_posix_requirements, required_socket_directory,
};
use iceoryx2_bb_system_types::path::Path;

/// Returns true when the host fulfills the POSIX requirements of iceoryx2-cal.
pub fn is_host_compliant() -> bool {
    does_system_satisfy_posix_requirements(ComplianceCheckMode::Silent)
}

/// Returns the directory unix domain sockets must be placed in, if the platform requires one.
pub fn socket_directory() -> Option<Path> {
    required_socket_directory()
}
