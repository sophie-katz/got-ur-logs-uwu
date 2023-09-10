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

use std::any::Any;

use crate::{
    HasDebugSeverity, HasDeveloperWarningSeverity, HasErrorSeverity, HasFatalSeverity,
    HasInfoSeverity, HasTraceSeverity, HasWarningSeverity, IsSeverity, Message,
};

pub struct Logger<Severity: IsSeverity> {
    min_severity: Severity,
}

impl<Severity: IsSeverity> Default for Logger<Severity> {
    fn default() -> Self {
        Self {
            min_severity: Severity::min(),
        }
    }
}

impl<Severity: IsSeverity> Logger<Severity> {
    pub fn global() -> &'static Self {
        static mut LOGGER: Option<Box<dyn Any>> = None;

        unsafe {
            if LOGGER.is_none() {
                LOGGER = Some(Box::<Logger<Severity>>::default())
            }

            LOGGER
                .as_ref()
                .expect("LOGGER should have been initialized above")
                .downcast_ref::<Logger<Severity>>()
                .expect("Global logger can only ever have one type")
        }
    }

    pub fn log_message(&self, message: Message<Severity>) {
        if message.severity >= self.min_severity {
            println!("[{}] {}", message.severity, message.text);
        }
    }

    pub fn log_with_severity(&self, severity: Severity, message: &str) {
        self.log_message(Message {
            severity,
            text: message.to_string(),
        });
    }

    pub fn log_trace(&self, message: &str)
    where
        Severity: HasTraceSeverity,
    {
        self.log_with_severity(Severity::trace_severity(), message);
    }

    pub fn log_debug(&self, message: &str)
    where
        Severity: HasDebugSeverity,
    {
        self.log_with_severity(Severity::debug_severity(), message);
    }

    pub fn log_developer_warning(&self, message: &str)
    where
        Severity: HasDeveloperWarningSeverity,
    {
        self.log_with_severity(Severity::developer_warning_severity(), message);
    }

    pub fn log_info(&self, message: &str)
    where
        Severity: HasInfoSeverity,
    {
        self.log_with_severity(Severity::info_severity(), message);
    }

    pub fn log_warning(&self, message: &str)
    where
        Severity: HasWarningSeverity,
    {
        self.log_with_severity(Severity::warning_severity(), message);
    }

    pub fn log_error(&self, message: &str)
    where
        Severity: HasErrorSeverity,
    {
        self.log_with_severity(Severity::error_severity(), message);
    }

    pub fn log_fatal(&self, message: &str)
    where
        Severity: HasFatalSeverity,
    {
        self.log_with_severity(Severity::fatal_severity(), message);
    }
}
