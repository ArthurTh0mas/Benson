// Copyright 2020-2021 Parity Technologies (UK) Ltd. and Annie Lai Investments Ltd.
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

//! Node-specific RPC methods for interaction with NFT module.

use std::{string::String, sync::Arc};

use benson_primitives::types::CollectionId;
use codec::Codec;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};

pub use self::gen_client::Client as NftClient;
pub use crml_nft_rpc_runtime_api::{self as runtime_api, NftApi as NftRuntimeApi};

/// NFT RPC methods.
#[rpc]
pub trait NftApi<TokenId, AccountId> {
	#[rpc(name = "nft_collectedTokens")]
	fn collected_tokens(&self, collection_id: String, who: AccountId) -> Result<Vec<TokenId>>;
}

/// Error type of this RPC api.
pub enum Error {
	/// The call to runtime failed.
	RuntimeError,
}

impl From<Error> for i64 {
	fn from(e: Error) -> i64 {
		match e {
			Error::RuntimeError => 1,
		}
	}
}

/// An implementation of NFT specific RPC methods.
pub struct Nft<C, T> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<T>,
}

impl<C, T> Nft<C, T> {
	/// Create new `Nft` with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Nft {
			client,
			_marker: Default::default(),
		}
	}
}

impl<C, Block, TokenId, AccountId> NftApi<TokenId, AccountId> for Nft<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	TokenId: Codec,
	C::Api: NftRuntimeApi<Block, CollectionId, TokenId, AccountId>,
	TokenId: Codec,
	AccountId: Codec,
{
	// `CollectionId` is a Vec<u8>, this was causing some issue for the js API
	// using String for better compatibility
	fn collected_tokens(&self, collection_id: String, who: AccountId) -> Result<Vec<TokenId>> {
		let api = self.client.runtime_api();
		let best = self.client.info().best_hash;
		let at = BlockId::hash(best);
		let collection_id_ = collection_id.into_bytes();
		api.collected_tokens(&at, collection_id_, who).map_err(|e| RpcError {
			code: ErrorCode::ServerError(Error::RuntimeError.into()),
			message: "Unable to query collection nfts.".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
}