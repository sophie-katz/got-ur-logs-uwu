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

//! A logging library for Rust.
//!
//! # Example
//!
//! ```
//! use got_ur_logs_uwu::{
//!     log_info,
//!     writers::ConsoleWriter,
//!     Logger,
//!     Message,
//!     Severity,
//!     formatters::Plaintext,
//! };
//!
//! // Initialize the global logger
//! Logger::<Severity, Message<Severity>>::global()
//!     .add_writer(
//!         ConsoleWriter::new_stdout(
//!             Plaintext::new_default()
//!         )
//!     );
//!
//! // Log a message
//! log_info!("hello, world");
//! ```

mod errors;
mod logger;
mod macros;
mod message;
mod severity;
mod traits;

pub mod formatters;
#[doc(hidden)]
pub mod private;
pub mod writers;

pub use errors::{Error, Result};
pub use logger::Logger;
pub use message::Message;
pub use severity::Severity;
pub use traits::{
    FromCoreFields, HasDebugSeverity, HasDeveloperWarningSeverity, HasErrorSeverity,
    HasFatalSeverity, HasInfoSeverity, HasSeverity, HasText, HasTraceSeverity, HasWarningSeverity,
    IsSeverity, Write,
};
