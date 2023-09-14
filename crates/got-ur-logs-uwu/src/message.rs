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

use crate::{FromCoreFields, HasSeverity, HasText, IsSeverity};

/// The default message type provided by `got-ur-logs-uwu`.
///
/// You can always define your own, but this one is provided by default.
///
/// # Example
///
/// ```
/// # use got_ur_logs_uwu::{Message, Severity, FromCoreFields};
/// #
/// Message::from_core_fields(Severity::Info, "hello, world");
/// ```
pub struct Message<Severity: IsSeverity> {
    pub(crate) _severity: Severity,
    pub(crate) _text: String,
}

impl<Severity: IsSeverity> HasSeverity<Severity> for Message<Severity> {
    fn severity(&self) -> &Severity {
        &self._severity
    }
}

impl<Severity: IsSeverity> HasText for Message<Severity> {
    fn text(&self) -> &str {
        self._text.as_str()
    }
}

impl<Severity: IsSeverity> FromCoreFields<Severity> for Message<Severity> {
    fn from_core_fields(severity: Severity, text: &str) -> Self {
        Message {
            _severity: severity,
            _text: text.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{FromCoreFields, Message, Severity};

    use super::*;

    #[test]
    fn core_fields() {
        let message = Message::from_core_fields(Severity::Debug, "test");

        assert_eq!(*message.severity(), Severity::Debug);
        assert_eq!(message.text(), "test");
    }
}
