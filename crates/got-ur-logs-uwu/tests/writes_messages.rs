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

use got_ur_logs_uwu::{formatters::Plaintext, writers::ConsoleWriter, Logger, Message, Severity};
use rstest::rstest;

enum LoggerType {
    DefaultGlobal,
    DefaultLocal,
}

#[derive(Default)]
struct TestContext {
    logger: Option<Logger<Severity, Message<Severity>>>,
}

impl TestContext {
    fn get_default_logger_global(&mut self) -> &mut Logger<Severity, Message<Severity>> {
        got_ur_logs_uwu::Logger::global()
    }

    fn get_default_logger_local(&mut self) -> &mut Logger<Severity, Message<Severity>> {
        if self.logger.is_none() {
            self.logger = Some(got_ur_logs_uwu::Logger::default());
        }

        self.logger.as_mut().unwrap()
    }

    pub fn get_logger(
        &mut self,
        logger_type: LoggerType,
    ) -> &mut Logger<Severity, Message<Severity>> {
        match logger_type {
            LoggerType::DefaultGlobal => self.get_default_logger_global(),
            LoggerType::DefaultLocal => self.get_default_logger_local(),
        }
    }
}

#[rstest]
fn no_writers_no_messages(
    #[values(LoggerType::DefaultGlobal, LoggerType::DefaultLocal)] logger_type: LoggerType,
) {
    let mut test_context = TestContext::default();

    test_context.get_logger(logger_type);
}

#[rstest]
fn one_writer_no_messages(
    #[values(LoggerType::DefaultGlobal, LoggerType::DefaultLocal)] logger_type: LoggerType,
) {
    let mut test_context = TestContext::default();

    let logger = test_context.get_logger(logger_type);

    let formatter = Plaintext::new_default();

    logger.add_writer(ConsoleWriter::new_stdout(formatter));
}
