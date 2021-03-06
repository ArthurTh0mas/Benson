// Copyright 2019-2020 Parity Technologies (UK) Ltd. and Annie Lai Investments Ltd.
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

//! Utilites to build a `TestClient` for `benson-runtime`.

use sp_runtime::BuildStorage;

/// Re-export test-client utilities.
pub use substrate_test_client::*;

/// Call executor for `benson-runtime` `TestClient`.
pub type Executor = sc_executor::NativeExecutor<benson_executor::Executor>;

/// Default backend type.
pub type Backend = sc_client_db::Backend<benson_primitives::types::Block>;

/// Test client type.
pub type Client = sc_client::Client<
	Backend,
	sc_client::LocalCallExecutor<Backend, Executor>,
	benson_primitives::types::Block,
	benson_runtime::RuntimeApi,
>;

/// Transaction for node-runtime.
pub type Transaction = sc_client_api::backend::TransactionFor<Backend, benson_primitives::types::Block>;

/// Genesis configuration parameters for `TestClient`.
#[derive(Default)]
pub struct GenesisParameters {
	support_changes_trie: bool,
}

impl substrate_test_client::GenesisInit for GenesisParameters {
	fn genesis_storage(&self) -> Storage {
		crate::genesis::config(self.support_changes_trie, None)
			.build_storage()
			.unwrap()
	}
}

/// A `test-runtime` extensions to `TestClientBuilder`.
pub trait TestClientBuilderExt: Sized {
	/// Create test client builder.
	fn new() -> Self;

	/// Build the test client.
	fn build(self) -> Client;
}

impl TestClientBuilderExt
	for substrate_test_client::TestClientBuilder<
		benson_primitives::types::Block,
		sc_client::LocalCallExecutor<Backend, Executor>,
		Backend,
		GenesisParameters,
	>
{
	fn new() -> Self {
		Self::default()
	}

	fn build(self) -> Client {
		self.build_with_native_executor(None).0
	}
}
