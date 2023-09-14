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
    traits::{Format, HasSeverity, HasText},
    IsSeverity, Result, Write,
};
use std::{io, marker::PhantomData};

enum ConsoleWriterDestination<'writer> {
    Stdout,
    Stderr,
    Writer(&'writer mut dyn io::Write),
}

/// A simple writer for console output.
///
/// # Example
///
/// ```
/// # use got_ur_logs_uwu::{
/// #     log_info,
/// #     writers::ConsoleWriter,
/// #     formatters::Plaintext,
/// #     Logger,
/// #     Message,
/// #     Severity,
/// # };
/// #
/// // Add a console writer to the global logger
/// Logger::<Severity, Message<Severity>>::global()
///     .add_writer(
///         ConsoleWriter::new_stdout(
///             Plaintext::new_default()
///         )
///     );
///
/// // Log a message
/// log_info!("hello, world"); // ← This will print to the console
/// ```
pub struct ConsoleWriter<
    'writer,
    SeverityType: IsSeverity,
    MessageType: HasSeverity<SeverityType> + HasText,
    FormatterType: Format<SeverityType, MessageType>,
> {
    destination: ConsoleWriterDestination<'writer>,
    formatter: FormatterType,
    severity_type_phantom: PhantomData<SeverityType>,
    message_type_phantom: PhantomData<MessageType>,
}

impl<
        'writer,
        SeverityType: IsSeverity,
        MessageType: HasSeverity<SeverityType> + HasText,
        FormatterType: Format<SeverityType, MessageType>,
    > ConsoleWriter<'writer, SeverityType, MessageType, FormatterType>
{
    /// Create a new console writer that writes to stdout.
    pub fn new_stdout(formatter: FormatterType) -> Self {
        Self {
            destination: ConsoleWriterDestination::Stdout,
            formatter,
            severity_type_phantom: PhantomData,
            message_type_phantom: PhantomData,
        }
    }

    /// Create a new console writer that writes to stderr.
    pub fn new_stderr(formatter: FormatterType) -> Self {
        Self {
            destination: ConsoleWriterDestination::Stderr,
            formatter,
            severity_type_phantom: PhantomData,
            message_type_phantom: PhantomData,
        }
    }

    /// Create a new console writer that writes to a custom writer.
    pub fn new_write(writer: &'writer mut dyn io::Write, formatter: FormatterType) -> Self {
        Self {
            destination: ConsoleWriterDestination::Writer(writer),
            formatter,
            severity_type_phantom: PhantomData,
            message_type_phantom: PhantomData,
        }
    }
}

impl<
        'writer,
        SeverityType: IsSeverity,
        MessageType: HasSeverity<SeverityType> + HasText,
        FormatterType: Format<SeverityType, MessageType>,
    > Write<SeverityType, MessageType>
    for ConsoleWriter<'writer, SeverityType, MessageType, FormatterType>
{
    fn write(&mut self, message: &MessageType) -> Result<()> {
        match self.destination {
            ConsoleWriterDestination::Stdout => self.formatter.format(message, &mut io::stdout()),
            ConsoleWriterDestination::Stderr => self.formatter.format(message, &mut io::stderr()),
            ConsoleWriterDestination::Writer(ref mut writer) => {
                self.formatter.format(message, writer)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{formatters::Plaintext, FromCoreFields, Message, Result, Severity};

    use super::*;

    #[test]
    fn smoke_stdout() -> Result<()> {
        let formatter = Plaintext::new_default();

        let mut writer = ConsoleWriter::new_stdout(formatter);

        writer.write(&Message::from_core_fields(Severity::Info, "hello, world"))
    }
}
