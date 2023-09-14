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

use std::collections::HashMap;

use crate::{traits::Format, HasSeverity, HasText, IsSeverity, Result};
use handlebars::Handlebars;

pub struct Plaintext {
    handlebars: Handlebars<'static>,
}

impl Plaintext {
    pub fn new<StringType: AsRef<str>>(template_string: StringType) -> Result<Self> {
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("plaintext", template_string)?;
        Ok(Self { handlebars })
    }

    pub fn new_default() -> Self {
        Self::new("[{{severity}}] {{text}}")
            .expect("template error when creating default formatter")
    }
}

impl<SeverityType: IsSeverity, MessageType: HasSeverity<SeverityType> + HasText>
    Format<SeverityType, MessageType> for Plaintext
{
    fn format(&mut self, message: &MessageType, writer: &mut dyn std::io::Write) -> Result<()> {
        let mut data = HashMap::new();

        data.insert("severity", message.severity().to_string());
        data.insert("text", message.text().to_owned());

        self.handlebars
            .render_to_write("plaintext", &data, writer)
            .map_err(|e| e.into())
    }
}
