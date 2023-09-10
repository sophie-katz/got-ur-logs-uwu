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

pub struct Message<Severity> {
    pub severity: Severity,
    pub text: String,
}

#[derive(Default)]
pub struct MessageBuilder<Severity: Default> {
    pub severity: Option<Severity>,
    pub text: Option<&'static str>,
}

impl<Severity: Default> MessageBuilder<Severity> {
    pub fn build(self) -> Message<Severity> {
        Message {
            severity: self.severity.expect("severity must be set"),
            text: self.text.expect("text must be set").to_owned(),
        }
    }
}
