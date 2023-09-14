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

use crate::{IsSeverity, Message};

/// A builder for [`Message`].
///
/// This is used by the macros to set fields as key-value pairs.
#[derive(Default)]
pub struct MessageBuilder<SeverityType: Default> {
    pub severity: Option<SeverityType>,
    pub text: Option<&'static str>,
}

impl<SeverityType: IsSeverity + Default> MessageBuilder<SeverityType> {
    pub fn build(self) -> Message<SeverityType> {
        Message {
            _severity: self.severity.expect("severity must be set"),
            _text: self.text.expect("text must be set").to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Severity;

    use super::*;

    #[test]
    fn build_message() {
        MessageBuilder::<Severity> {
            severity: Some(Severity::Info),
            text: Some("test"),
        }
        .build();
    }
}
