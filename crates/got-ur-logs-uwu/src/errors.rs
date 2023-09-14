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

use std::{io, result};
use thiserror::Error;

/// Crate error type
#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    IOError(io::Error),
    #[error("Handlebars render error: {0}")]
    HandlebarsRenderError(Box<handlebars::RenderError>),
    #[error("Handlebars template error: {0}")]
    HandlebarsTemplateError(Box<handlebars::TemplateError>),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::IOError(error)
    }
}

impl From<handlebars::RenderError> for Error {
    fn from(error: handlebars::RenderError) -> Self {
        Self::HandlebarsRenderError(Box::new(error))
    }
}

impl From<handlebars::TemplateError> for Error {
    fn from(error: handlebars::TemplateError) -> Self {
        Self::HandlebarsTemplateError(Box::new(error))
    }
}

/// Crate result type
pub type Result<Value> = result::Result<Value, Error>;
