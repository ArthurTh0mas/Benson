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

use codec::{Decode, Encode};
use frame_support::Hashable;
use sc_executor::error::Result;
use sc_executor::{NativeExecutor, WasmExecutionMethod};
use sp_core::{
	traits::{CodeExecutor, RuntimeCode},
	NativeOrEncoded, NeverNativeValue,
};
use sp_runtime::{
	traits::{BlakeTwo256, Header as HeaderT},
	ApplyExtrinsicResult,
};
use sp_state_machine::TestExternalities as CoreTestExternalities;

use benson_executor::Executor;
use benson_primitives::types::{BlockNumber, Hash};
use benson_runtime::{
	constants::{asset::SPENDING_ASSET_ID, currency::*},
	Block, BuildStorage, CheckedExtrinsic, Header, Runtime, UncheckedExtrinsic,
};
use benson_testing::keyring::*;
use sp_externalities::Externalities;

/// The wasm runtime code.
///
/// `compact` since it is after post-processing with wasm-gc which performs tree-shaking thus
/// making the binary slimmer. There is a convention to use compact version of the runtime
/// as canonical. This is why `native_executor_instance` also uses the compact version of the
/// runtime.
pub const COMPACT_CODE: &[u8] = benson_runtime::WASM_BINARY;

pub const GENESIS_HASH: [u8; 32] = [69u8; 32];

pub const VERSION: u32 = benson_runtime::VERSION.spec_version;

pub type TestExternalities<H> = CoreTestExternalities<H, u64>;

pub fn sign(xt: CheckedExtrinsic) -> UncheckedExtrinsic {
	benson_testing::keyring::sign(xt, VERSION, GENESIS_HASH)
}

pub fn default_transfer_call() -> pallet_generic_asset::Call<Runtime> {
	pallet_generic_asset::Call::transfer::<Runtime>(SPENDING_ASSET_ID, bob().into(), 69 * DOLLARS)
}

pub fn from_block_number(n: u32) -> Header {
	Header::new(
		n,
		Default::default(),
		Default::default(),
		[69; 32].into(),
		Default::default(),
	)
}

pub fn executor() -> NativeExecutor<Executor> {
	NativeExecutor::new(WasmExecutionMethod::Interpreted, None, 8)
}

pub fn executor_call<
	R: Decode + Encode + PartialEq,
	NC: FnOnce() -> std::result::Result<R, String> + std::panic::UnwindSafe,
>(
	t: &mut TestExternalities<BlakeTwo256>,
	method: &str,
	data: &[u8],
	use_native: bool,
	native_call: Option<NC>,
) -> (Result<NativeOrEncoded<R>>, bool) {
	let mut t = t.ext();

	let code = t.storage(sp_core::storage::well_known_keys::CODE).unwrap();
	let heap_pages = t.storage(sp_core::storage::well_known_keys::HEAP_PAGES);
	let runtime_code = RuntimeCode {
		code_fetcher: &sp_core::traits::WrappedRuntimeCode(code.as_slice().into()),
		hash: sp_core::blake2_256(&code).to_vec(),
		heap_pages: heap_pages.and_then(|hp| Decode::decode(&mut &hp[..]).ok()),
	};

	executor().call::<R, NC>(&mut t, &runtime_code, method, data, use_native, native_call)
}

pub fn new_test_ext(code: &[u8], support_changes_trie: bool) -> TestExternalities<BlakeTwo256> {
	let mut ext = TestExternalities::new_with_code(
		code,
		benson_testing::genesis::config(support_changes_trie, Some(code))
			.build_storage()
			.unwrap(),
	);
	ext.changes_trie_storage()
		.insert(0, GENESIS_HASH.into(), Default::default());
	ext
}

/// Construct a fake block.
///
/// `extrinsics` must be a list of valid extrinsics, i.e. none of the extrinsics for example
/// can report `ExhaustResources`. Otherwise, this function panics.
pub fn construct_block(
	env: &mut TestExternalities<BlakeTwo256>,
	number: BlockNumber,
	parent_hash: Hash,
	extrinsics: Vec<CheckedExtrinsic>,
) -> (Vec<u8>, Hash) {
	use sp_trie::{trie_types::Layout, TrieConfiguration};

	// sign extrinsics.
	let extrinsics = extrinsics.into_iter().map(sign).collect::<Vec<_>>();

	// calculate the header fields that we can.
	let extrinsics_root = Layout::<BlakeTwo256>::ordered_trie_root(extrinsics.iter().map(Encode::encode))
		.to_fixed_bytes()
		.into();

	let header = Header {
		parent_hash,
		number,
		extrinsics_root,
		state_root: Default::default(),
		digest: Default::default(),
	};

	// execute the block to get the real header.
	executor_call::<NeverNativeValue, fn() -> _>(env, "Core_initialize_block", &header.encode(), true, None)
		.0
		.unwrap();

	for extrinsic in extrinsics.iter() {
		// Try to apply the `extrinsic`. It should be valid, in the sense that it passes
		// all pre-inclusion checks.
		let r = executor_call::<NeverNativeValue, fn() -> _>(
			env,
			"BlockBuilder_apply_extrinsic",
			&extrinsic.encode(),
			true,
			None,
		)
		.0
		.expect("application of an extrinsic failed")
		.into_encoded();
		match ApplyExtrinsicResult::decode(&mut &r[..]).expect("apply result deserialization failed") {
			Ok(_) => {}
			Err(e) => panic!("Applying extrinsic failed: {:?}", e),
		}
	}

	let header =
		match executor_call::<NeverNativeValue, fn() -> _>(env, "BlockBuilder_finalize_block", &[0u8; 0], true, None)
			.0
			.unwrap()
		{
			NativeOrEncoded::Native(_) => unreachable!(),
			NativeOrEncoded::Encoded(h) => Header::decode(&mut &h[..]).unwrap(),
		};

	let hash = header.blake2_256();
	(Block { header, extrinsics }.encode(), hash.into())
}
