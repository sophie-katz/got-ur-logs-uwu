// Copyright (c) 2023 Sophie Katz
//
// This file is part of got-ur-logs-uwu.
//
// got-ur-logs-uwu is free software: you can redistribute it and/or modify it under the terms of the
// GNU General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// got-ur-logs-uwu is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with got-ur-logs-uwu. If
// not, see <https://www.gnu.org/licenses/>.

use crate::{
    HasDebugSeverity, HasDeveloperWarningSeverity, HasErrorSeverity, HasFatalSeverity,
    HasInfoSeverity, HasTraceSeverity, HasWarningSeverity, IsSeverity,
};
use strum_macros::Display;

/// The default severity type provided by `got-ur-logs-uwu`.
///
/// You can always define your own, but this one is provided by default.
#[derive(Debug, Display, PartialEq, PartialOrd)]
pub enum Severity {
    /// Trace messages are used for highly verbose tracing of code that is only applicable for
    /// developers tracing program execution.
    #[strum(serialize = "trace")]
    Trace,
    /// Debug messages are those that are useful to developers, but are too verbose for other use.
    #[strum(serialize = "debug")]
    Debug,
    /// Developer warnings are warnings that are useful to developers, but are not applicable for
    /// other users.
    #[strum(serialize = "dev warning")]
    DeveloperWarning,
    /// Informative messages to all users.
    #[strum(serialize = "info")]
    Info,
    /// Warning messages to all users, indicating that something may be wrong but the program can
    /// safely continue.
    #[strum(serialize = "warning")]
    Warning,
    /// Error messages to all users, indicating that something went wrong but the program can
    /// continue.
    #[strum(serialize = "error")]
    Error,
    /// Fatal messages to all users, indicating that something went wrong and the program cannot
    /// continue.
    #[strum(serialize = "fatal")]
    Fatal,
}

impl Default for Severity {
    fn default() -> Self {
        Self::Info
    }
}

impl IsSeverity for Severity {
    fn min() -> Self {
        Self::Trace
    }

    fn max() -> Self {
        Self::Fatal
    }
}

impl HasTraceSeverity for Severity {
    fn trace_severity() -> Self {
        Self::Trace
    }
}

impl HasDebugSeverity for Severity {
    fn debug_severity() -> Self {
        Self::Debug
    }
}

impl HasDeveloperWarningSeverity for Severity {
    fn developer_warning_severity() -> Self {
        Self::DeveloperWarning
    }
}

impl HasInfoSeverity for Severity {
    fn info_severity() -> Self {
        Self::Info
    }
}

impl HasWarningSeverity for Severity {
    fn warning_severity() -> Self {
        Self::Warning
    }
}

impl HasErrorSeverity for Severity {
    fn error_severity() -> Self {
        Self::Error
    }
}

impl HasFatalSeverity for Severity {
    fn fatal_severity() -> Self {
        Self::Fatal
    }
}
