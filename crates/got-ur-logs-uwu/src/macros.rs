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

#[macro_export]
macro_rules! log_message {
    ($($field:ident = $value:expr),* $(,)?) => {
        $crate::Logger::global().log_message(
            #[allow(clippy::needless_update)]
            $crate::MessageBuilder {
                $(
                    $field: Some($value),
                )*
                ..std::default::Default::default()
            }.build()
        )
    };
}

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
