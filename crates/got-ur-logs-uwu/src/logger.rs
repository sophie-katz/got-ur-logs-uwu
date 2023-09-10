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

use std::{any::Any, cell::RefCell};

use crate::{
    FromCoreFields, HasDebugSeverity, HasDeveloperWarningSeverity, HasErrorSeverity,
    HasFatalSeverity, HasInfoSeverity, HasSeverity, HasText, HasTraceSeverity, HasWarningSeverity,
    IsSeverity, Write,
};

pub struct Logger<Severity: IsSeverity, Message: HasSeverity<Severity> + HasText> {
    min_severity: Severity,
    writers: Vec<RefCell<Box<dyn Write<Severity, Message>>>>,
}

impl<Severity: IsSeverity, Message: HasSeverity<Severity> + HasText> Default
    for Logger<Severity, Message>
{
    fn default() -> Self {
        Self {
            min_severity: Severity::min(),
            writers: Vec::new(),
        }
    }
}

impl<Severity: IsSeverity, Message: HasSeverity<Severity> + HasText> Logger<Severity, Message> {
    pub fn global() -> &'static mut Self {
        static mut LOGGER: Option<Box<dyn Any>> = None;

        unsafe {
            if LOGGER.is_none() {
                LOGGER = Some(Box::<Self>::default())
            }

            LOGGER
                .as_mut()
                .expect("LOGGER should have been initialized above")
                .downcast_mut::<Self>()
                .expect("Global logger can only ever have one type")
        }
    }

    pub fn add_writer<Writer: 'static + Write<Severity, Message>>(&mut self, writer: Writer) {
        self.writers.push(RefCell::new(Box::new(writer)));
    }

    pub fn log_message(&self, message: Message) {
        if message.severity() >= &self.min_severity {
            // println!("[{}] {}", message.severity(), message.text());
            for writer in &self.writers {
                writer
                    .borrow_mut()
                    .write(&message)
                    .expect("Failed to write message");
            }
        }
    }

    pub fn log_with_severity(&self, severity: Severity, text: &str)
    where
        Message: FromCoreFields<Severity>,
    {
        self.log_message(Message::from_core_fields(severity, text));
    }

    pub fn log_trace(&self, text: &str)
    where
        Message: FromCoreFields<Severity>,
        Severity: HasTraceSeverity,
    {
        self.log_with_severity(Severity::trace_severity(), text);
    }

    pub fn log_debug(&self, text: &str)
    where
        Message: FromCoreFields<Severity>,
        Severity: HasDebugSeverity,
    {
        self.log_with_severity(Severity::debug_severity(), text);
    }

    pub fn log_developer_warning(&self, text: &str)
    where
        Message: FromCoreFields<Severity>,
        Severity: HasDeveloperWarningSeverity,
    {
        self.log_with_severity(Severity::developer_warning_severity(), text);
    }

    pub fn log_info(&self, text: &str)
    where
        Message: FromCoreFields<Severity>,
        Severity: HasInfoSeverity,
    {
        self.log_with_severity(Severity::info_severity(), text);
    }

    pub fn log_warning(&self, text: &str)
    where
        Message: FromCoreFields<Severity>,
        Severity: HasWarningSeverity,
    {
        self.log_with_severity(Severity::warning_severity(), text);
    }

    pub fn log_error(&self, text: &str)
    where
        Message: FromCoreFields<Severity>,
        Severity: HasErrorSeverity,
    {
        self.log_with_severity(Severity::error_severity(), text);
    }

    pub fn log_fatal(&self, text: &str)
    where
        Message: FromCoreFields<Severity>,
        Severity: HasFatalSeverity,
    {
        self.log_with_severity(Severity::fatal_severity(), text);
    }
}
