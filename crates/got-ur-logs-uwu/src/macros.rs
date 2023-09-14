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

#[allow(unused_imports)]
use crate::private::MessageBuilder; // Used by doc comment

/// Logs a message to the default global logger.
///
/// # Arguments
///
/// Takes a comma-separated list of key-value pairs. The allowed keys are:
/// * `severity` - The severity of the message
/// * `text` - The text content of the message
///
/// # Example
///
/// ```
/// # use got_ur_logs_uwu::{
/// #     Severity,
/// #     log_message,
/// # };
/// #
/// log_message!(severity = Severity::Info, text = "hello, world");
/// ```
#[macro_export]
macro_rules! log_message {
    ($($field:ident = $value:expr),* $(,)?) => {
        $crate::Logger::global().log_message(
            #[allow(clippy::needless_update)]
            $crate::private::MessageBuilder {
                $(
                    $field: Some($value),
                )*
                ..std::default::Default::default()
            }.build()
        )
    };
}

/// Logs a message with core fields to the default global logger.
///
/// # Arguments
///
/// Takes two positional arguments:
/// * `severity` - The severity of the message
/// * `text` - The text content of the message
///
/// Additionally, takes a comma-separated list of key-value pairs. The keys correspond to the fields
/// of the [`MessageBuilder`] type.
///
/// # Example
///
/// ```
/// # use got_ur_logs_uwu::{
/// #     Severity,
/// #     log_with_severity,
/// # };
/// #
/// log_with_severity!(Severity::Info, "hello, world");
/// ```
#[macro_export]
macro_rules! log_with_severity {
    ($severity:expr, $text:expr $(, $field:ident = $value:expr)*) => {
        $crate::log_message!(
            severity = $severity,
            text = $text,
            $(, $field = $value)*
        )
    };
}

/// Logs a trace message to the default global logger.
///
/// # Arguments
///
/// Takes one positional argument:
/// * `text` - The text content of the message
///
/// Additionally, takes a comma-separated list of key-value pairs. The keys correspond to the fields
/// of the [`MessageBuilder`] type.
///
/// # Example
///
/// ```
/// # use got_ur_logs_uwu::{
/// #     Severity,
/// #     log_trace,
/// # };
/// #
/// log_trace!("hello, world");
/// ```
#[macro_export]
macro_rules! log_trace {
    ($text:expr $(, $field:ident = $value:expr)*) => {
        $crate::log_with_severity!(
            $crate::Severity::Trace,
            $text
            $(, $field = $value)*
        )
    };
}

/// Logs a debug message to the default global logger.
///
/// # Arguments
///
/// Takes one positional argument:
/// * `text` - The text content of the message
///
/// Additionally, takes a comma-separated list of key-value pairs. The keys correspond to the fields
/// of the [`MessageBuilder`] type.
///
/// # Example
///
/// ```
/// # use got_ur_logs_uwu::{
/// #     Severity,
/// #     log_debug,
/// # };
/// #
/// log_debug!("hello, world");
/// ```
#[macro_export]
macro_rules! log_debug {
    ($text:expr $(, $field:ident = $value:expr)*) => {
        $crate::log_with_severity!(
            $crate::Severity::Debug,
            $text
            $(, $field = $value)*
        )
    };
}

/// Logs a developer warning message to the default global logger.
///
/// # Arguments
///
/// Takes one positional argument:
/// * `text` - The text content of the message
///
/// Additionally, takes a comma-separated list of key-value pairs. The keys correspond to the fields
/// of the [`MessageBuilder`] type.
///
/// # Example
///
/// ```
/// # use got_ur_logs_uwu::{
/// #     Severity,
/// #     log_developer_warning,
/// # };
/// #
/// log_developer_warning!("hello, world");
/// ```
#[macro_export]
macro_rules! log_developer_warning {
    ($text:expr $(, $field:ident = $value:expr)*) => {
        $crate::log_with_severity!(
            $crate::Severity::DeveloperWarning,
            $text
            $(, $field = $value)*
        )
    };
}

/// Logs an info message to the default global logger.
///
/// # Arguments
///
/// Takes one positional argument:
/// * `text` - The text content of the message
///
/// Additionally, takes a comma-separated list of key-value pairs. The keys correspond to the fields
/// of the [`MessageBuilder`] type.
///
/// # Example
///
/// ```
/// # use got_ur_logs_uwu::{
/// #     Severity,
/// #     log_info,
/// # };
/// #
/// log_info!("hello, world");
/// ```
#[macro_export]
macro_rules! log_info {
    ($text:expr $(, $field:ident = $value:expr)*) => {
        $crate::log_with_severity!(
            $crate::Severity::Info,
            $text
            $(, $field = $value)*
        )
    };
}

/// Logs a warning message to the default global logger.
///
/// # Arguments
///
/// Takes one positional argument:
/// * `text` - The text content of the message
///
/// Additionally, takes a comma-separated list of key-value pairs. The keys correspond to the fields
/// of the [`MessageBuilder`] type.
///
/// # Example
///
/// ```
/// # use got_ur_logs_uwu::{
/// #     Severity,
/// #     log_warning,
/// # };
/// #
/// log_warning!("hello, world");
/// ```
#[macro_export]
macro_rules! log_warning {
    ($text:expr $(, $field:ident = $value:expr)*) => {
        $crate::log_with_severity!(
            $crate::Severity::Warning,
            $text
            $(, $field = $value)*
        )
    };
}

/// Logs an error message to the default global logger.
///
/// # Arguments
///
/// Takes one positional argument:
/// * `text` - The text content of the message
///
/// Additionally, takes a comma-separated list of key-value pairs. The keys correspond to the fields
/// of the [`MessageBuilder`] type.
///
/// # Example
///
/// ```
/// # use got_ur_logs_uwu::{
/// #     Severity,
/// #     log_error,
/// # };
/// #
/// log_error!("hello, world");
/// ```
#[macro_export]
macro_rules! log_error {
    ($text:expr $(, $field:ident = $value:expr)*) => {
        $crate::log_with_severity!(
            $crate::Severity::Error,
            $text
            $(, $field = $value)*
        )
    };
}

/// Logs a fatal error message to the default global logger.
///
/// # Arguments
///
/// Takes one positional argument:
/// * `text` - The text content of the message
///
/// Additionally, takes a comma-separated list of key-value pairs. The keys correspond to the fields
/// of the [`MessageBuilder`] type.
///
/// # Example
///
/// ```
/// # use got_ur_logs_uwu::{
/// #     Severity,
/// #     log_fatal,
/// # };
/// #
/// log_fatal!("hello, world");
/// ```
#[macro_export]
macro_rules! log_fatal {
    ($text:expr $(, $field:ident = $value:expr)*) => {
        $crate::log_with_severity!(
            $crate::Severity::Fatal,
            $text
            $(, $field = $value)*
        )
    };
}
