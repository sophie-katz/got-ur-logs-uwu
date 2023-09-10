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

use std::fmt::Display;

pub trait IsSeverity: PartialEq + PartialOrd + Display {
    fn min() -> Self;
    fn max() -> Self;
}

pub trait HasTraceSeverity {
    fn trace_severity() -> Self;
}

pub trait HasDebugSeverity {
    fn debug_severity() -> Self;
}

pub trait HasDeveloperWarningSeverity {
    fn developer_warning_severity() -> Self;
}

pub trait HasInfoSeverity {
    fn info_severity() -> Self;
}

pub trait HasWarningSeverity {
    fn warning_severity() -> Self;
}

pub trait HasErrorSeverity {
    fn error_severity() -> Self;
}

pub trait HasFatalSeverity {
    fn fatal_severity() -> Self;
}

pub trait Write {}
