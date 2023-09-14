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

use std::{
    any::Any,
    sync::{Arc, Mutex},
};

use crate::{
    FromCoreFields, HasDebugSeverity, HasDeveloperWarningSeverity, HasErrorSeverity,
    HasFatalSeverity, HasInfoSeverity, HasSeverity, HasText, HasTraceSeverity, HasWarningSeverity,
    IsSeverity, Write,
};

/// The logger is the main interface for the library.
///
/// Any messages that are logged go through the logger instance. It can be configured with any
/// number of writers, which are responsible for actually writing the messages to their
/// destinations.
///
/// # Example
///
/// ```
/// use got_ur_logs_uwu::{writers::ConsoleWriter, Logger, Message, Severity, formatters::Plaintext};
///
/// // Create logger with default settings
/// //
/// // This will have no writers
/// let mut logger = Logger::<Severity, Message<Severity>>::default();
///
/// // Add a console writer so we can see the output
/// logger.add_writer(
///     ConsoleWriter::new_stdout(
///         Plaintext::new_default()
///     )
/// );
///
/// // Log an info message
/// logger.log_info("hello, world"); // ← This will print to the console
/// ```
pub struct Logger<Severity: IsSeverity, Message: HasSeverity<Severity> + HasText> {
    min_severity: Severity,
    // writers: Vec<RefCell<Rc<dyn Write<Severity, Message>>>>,
    writers: Vec<Arc<Mutex<dyn Write<Severity, Message>>>>,
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
    /// Get the default global logger instance.
    ///
    /// This is used by the macros to log messages.
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

    /// Adds a writer to the logger.
    ///
    /// # Example
    ///
    /// ```
    /// # use got_ur_logs_uwu::{
    /// #     writers::ConsoleWriter, Logger, Message, Severity, formatters::Plaintext
    /// # };
    /// #
    /// # let mut logger = Logger::<Severity, Message<Severity>>::default();
    /// #
    /// logger.add_writer(
    ///     ConsoleWriter::new_stdout(
    ///         Plaintext::new_default()
    ///     )
    /// );
    /// ```
    pub fn add_writer<Writer: 'static + Write<Severity, Message>>(&mut self, writer: Writer) {
        self.writers.push(Arc::new(Mutex::new(writer)));
    }

    /// Adds a shared writer instance to the logger.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::sync::{Arc, Mutex};
    /// # use got_ur_logs_uwu::{
    /// #     writers::ConsoleWriter, Logger, Message, Severity, formatters::Plaintext
    /// # };
    /// #
    /// # let mut logger = Logger::<Severity, Message<Severity>>::default();
    /// #
    /// let writer = Arc::new(Mutex::new(
    ///     ConsoleWriter::new_stdout(
    ///         Plaintext::new_default()
    ///     )
    /// ));
    ///
    /// logger.add_writer_shared(
    ///     writer
    /// );
    /// ```
    pub fn add_writer_shared(&mut self, writer: Arc<Mutex<dyn Write<Severity, Message>>>) {
        self.writers.push(writer);
    }

    /// Logs a message object.
    ///
    /// # Arguments
    ///
    /// * `message` - The message object that will be passed along to the writers
    pub fn log_message(&self, message: Message) {
        if message.severity() >= &self.min_severity {
            for writer in &self.writers {
                writer
                    .lock()
                    .unwrap()
                    .write(&message)
                    .expect("Failed to write message");
            }
        }
    }

    /// Logs a message with the core fields set
    ///
    /// # Arguments
    ///
    /// * `severity` - The severity of the message
    /// * `text` - The text content of the message
    pub fn log_with_severity(&self, severity: Severity, text: &str)
    where
        Message: FromCoreFields<Severity>,
    {
        self.log_message(Message::from_core_fields(severity, text));
    }

    /// Logs a trace message
    ///
    /// # Arguments
    ///
    /// * `text` - The text content of the message
    pub fn log_trace(&self, text: &str)
    where
        Message: FromCoreFields<Severity>,
        Severity: HasTraceSeverity,
    {
        self.log_with_severity(Severity::trace_severity(), text);
    }

    /// Logs a debug message
    ///
    /// # Arguments
    ///
    /// * `text` - The text content of the message
    pub fn log_debug(&self, text: &str)
    where
        Message: FromCoreFields<Severity>,
        Severity: HasDebugSeverity,
    {
        self.log_with_severity(Severity::debug_severity(), text);
    }

    /// Logs a developer warning message
    ///
    /// # Arguments
    ///
    /// * `text` - The text content of the message
    pub fn log_developer_warning(&self, text: &str)
    where
        Message: FromCoreFields<Severity>,
        Severity: HasDeveloperWarningSeverity,
    {
        self.log_with_severity(Severity::developer_warning_severity(), text);
    }

    /// Logs an info message
    ///
    /// # Arguments
    ///
    /// * `text` - The text content of the message
    pub fn log_info(&self, text: &str)
    where
        Message: FromCoreFields<Severity>,
        Severity: HasInfoSeverity,
    {
        self.log_with_severity(Severity::info_severity(), text);
    }

    /// Logs a warning message
    ///
    /// # Arguments
    ///
    /// * `text` - The text content of the message
    pub fn log_warning(&self, text: &str)
    where
        Message: FromCoreFields<Severity>,
        Severity: HasWarningSeverity,
    {
        self.log_with_severity(Severity::warning_severity(), text);
    }

    /// Logs an error message
    ///
    /// # Arguments
    ///
    /// * `text` - The text content of the message
    pub fn log_error(&self, text: &str)
    where
        Message: FromCoreFields<Severity>,
        Severity: HasErrorSeverity,
    {
        self.log_with_severity(Severity::error_severity(), text);
    }

    /// Logs a fatal error message
    ///
    /// # Arguments
    ///
    /// * `text` - The text content of the message
    pub fn log_fatal(&self, text: &str)
    where
        Message: FromCoreFields<Severity>,
        Severity: HasFatalSeverity,
    {
        self.log_with_severity(Severity::fatal_severity(), text);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::{traits::MockWrite, Message, Severity};

    use super::*;

    // fn add_mock_writer<'writers>(logger: &'writers mut Logger<Severity, Message<Severity>>) {
    //     let mut writer = MockWrite::<Severity, Message<Severity>>::new();

    //     writer.expect_write().times(9).returning(|_| Ok(()));

    //     logger.add_writer(&mut writer);
    // }

    fn test_logger(logger: &Logger<Severity, Message<Severity>>) {
        logger.log_message(Message::from_core_fields(Severity::Info, "test"));
        logger.log_with_severity(Severity::Info, "test");
        logger.log_trace("test");
        logger.log_debug("test");
        logger.log_developer_warning("test");
        logger.log_info("test");
        logger.log_warning("test");
        logger.log_error("test");
        logger.log_fatal("test");
    }

    #[test]
    fn no_op_smoke() {
        let logger = Logger::<Severity, Message<Severity>>::default();

        test_logger(&logger);
    }

    #[test]
    fn writes_messages_local() {
        let mut logger = Logger::<Severity, Message<Severity>>::default();

        let mut writer = MockWrite::<Severity, Message<Severity>>::new();

        writer.expect_write().times(9).returning(|_| Ok(()));

        logger.add_writer(writer);

        test_logger(&logger);
    }

    #[test]
    fn writes_messages_global() {
        let writer = Arc::new(Mutex::new(
            MockWrite::<Severity, Message<Severity>>::default(),
        ));

        writer
            .lock()
            .unwrap()
            .expect_write()
            .times(9)
            .returning(|_| Ok(()));

        Logger::<Severity, Message<Severity>>::global().add_writer_shared(writer);

        test_logger(Logger::<Severity, Message<Severity>>::global());
    }
}
