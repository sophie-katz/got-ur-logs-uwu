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
    traits::{HasSeverity, HasText},
    IsSeverity, Result, Write,
};

pub struct ConsoleWriter;

impl<Severity: IsSeverity, Message: HasSeverity<Severity> + HasText> Write<Severity, Message>
    for ConsoleWriter
{
    fn write(&mut self, message: &Message) -> Result<()> {
        println!("[{}] {}", message.severity(), message.text());
        Ok(())
    }
}
