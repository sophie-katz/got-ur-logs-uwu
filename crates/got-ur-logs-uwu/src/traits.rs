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

use crate::Result;
use mockall::automock;
use std::{fmt::Display, io};

#[allow(unused_imports)]
use crate::Severity; // Used for doc comments

/// A trait implemented by all severity types.
pub trait IsSeverity: PartialEq + PartialOrd + Display {
    /// Get the minimum severity.
    ///
    /// This is the most verbose. By default this is a [`Severity::Trace`] message.
    fn min() -> Self;

    /// Get the maximum severity.
    ///
    /// This is the most critical. By default this is a [`Severity::Fatal`] message.
    fn max() -> Self;
}

/// A trait implemented by severity types that have a trace level.
pub trait HasTraceSeverity {
    /// Gets the trace level.
    fn trace_severity() -> Self;
}

/// A trait implemented by severity types that have a debug level.
pub trait HasDebugSeverity {
    /// Gets the debug level.
    fn debug_severity() -> Self;
}

/// A trait implemented by severity types that have a developer warning level.
pub trait HasDeveloperWarningSeverity {
    /// Gets the developer warning level.
    fn developer_warning_severity() -> Self;
}

/// A trait implemented by severity types that have an info level.
pub trait HasInfoSeverity {
    /// Gets the info level.
    fn info_severity() -> Self;
}

/// A trait implemented by severity types that have a warning level.
pub trait HasWarningSeverity {
    /// Gets the warning level.
    fn warning_severity() -> Self;
}

/// A trait implemented by severity types that have an error level.
pub trait HasErrorSeverity {
    /// Gets the error level.
    fn error_severity() -> Self;
}

/// A trait implemented by severity types that have a fatal error level.
pub trait HasFatalSeverity {
    /// Gets the fatal error level.
    fn fatal_severity() -> Self;
}

/// A trait implemented by all message types, indicating that they have severity levels.
pub trait HasSeverity<Severity: IsSeverity> {
    /// Get the severity of the message.
    fn severity(&self) -> &Severity;
}

/// A trait implemented by all message types, indicating that they have text content.
pub trait HasText {
    fn text(&self) -> &str;
}

/// A trait implemented by message types so that they can be constructed by macros.
///
/// It is essentially a constructor for the message object which is provided with just the core
/// fields.
pub trait FromCoreFields<Severity: IsSeverity> {
    /// Construct a new message from the core fields.
    ///
    /// # Arguments
    ///
    /// * `severity` - The severity of the message
    /// * `text` - The text content of the message
    fn from_core_fields(severity: Severity, text: &str) -> Self;
}

/// A trait implemented by all log writers.
///
/// Allows them to accept messages so that they may be written.
#[automock]
pub trait Write<Severity: IsSeverity, Message: HasSeverity<Severity> + HasText> {
    /// Writes a given message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to write
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the message was successfully written, or an error if it was not.
    fn write(&mut self, message: &Message) -> Result<()>;
}

pub trait Format<Severity: IsSeverity, Message: HasSeverity<Severity> + HasText> {
    fn format(&mut self, message: &Message, writer: &mut dyn io::Write) -> Result<()>;
}
