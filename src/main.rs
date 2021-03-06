// Copyright 2018-2020 Parity Technologies (UK) Ltd. and Annie Lai Investments Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! Benson Box CLI

#![warn(missing_docs)]

use cli::{CliResult, VersionInfo};

fn main() -> CliResult<()> {
	let version = VersionInfo {
		name: "Benson Box",
		commit: env!("VERGEN_SHA_SHORT"),
		version: env!("CARGO_PKG_VERSION"),
		executable_name: "benson",
		author: "Annie Lai <ng8eke@163.com>",
		description: "Benson Box by Annie Lai Investments UNlimited",
		support_url: "https://github.com/ng8eke/benson/issues/new",
		copyright_start_year: 2018,
	};

	cli::run(std::env::args(), version)
}
